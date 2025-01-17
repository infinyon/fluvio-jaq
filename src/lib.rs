use std::sync::OnceLock;

use eyre::ContextCompat;
use jaq_core::load::{Arena, File, Loader};
use jaq_core::{Compiler, Ctx, Filter, Native, RcIter};
use jaq_json::Val;
use serde_json::Value;

use fluvio_smartmodule::dataplane::smartmodule::SmartModuleInitError;
use fluvio_smartmodule::{
    dataplane::smartmodule::SmartModuleExtraParams, smartmodule, RecordData, Result,
    SmartModuleRecord,
};

type JqFilter = Filter<Native<Val>>;

static FILTER: OnceLock<JqFilter> = OnceLock::new();

const PARAM_NAME: &str = "filter";

#[smartmodule(init)]
fn init(params: SmartModuleExtraParams) -> Result<()> {
    if let Some(raw_spec) = params.get(PARAM_NAME) {
        let program = File {
            code: raw_spec.as_str(),
            path: (),
        };

        let loader = Loader::new(jaq_std::defs().chain(jaq_json::defs()));
        let arena = Arena::default();
        let modules = loader.load(&arena, program).map_err(|err| {
            let output_err = err
                .iter()
                .map(|err| format!("{:#?}", err))
                .collect::<Vec<String>>()
                .join("\n");
            eyre::Report::msg(output_err)
        })?;

        let compiler = Compiler::default().with_funs(jaq_std::funs().chain(jaq_json::funs()));

        let filter = compiler
            .compile(modules)
            .map_err(|err| eyre::Report::msg(format!("{:#?}", err)))?;

        FILTER
            .set(filter)
            .map_err(|_| eyre::Report::msg("jq spec is already initialized"))?;

        Ok(())
    } else {
        Err(SmartModuleInitError::MissingParam(PARAM_NAME.to_string()).into())
    }
}

#[smartmodule(map)]
pub fn map(record: &SmartModuleRecord) -> Result<(Option<RecordData>, RecordData)> {
    let filter = FILTER.get().wrap_err("jolt spec is not initialized")?;

    let key = record.key.clone();
    let json: Value = serde_json::from_slice(record.value.as_ref())?;
    let inputs = RcIter::new(core::iter::empty());
    let mut out = filter.run((Ctx::new([], &inputs), Val::from(json)));
    let mut out_json: Vec<Value> = vec![];
    loop {
        match out.next() {
            Some(Ok(val)) => {
                out_json.push(val.into());
            }
            Some(Err(err)) => return Err(eyre::Report::msg(format!("{:#?}", err))),
            None => {
                break;
            }
        }
    }

    if out_json.len() == 1 {
        Ok((key, serde_json::to_vec(&out_json[0])?.into()))
    } else {
        Ok((key, serde_json::to_vec(&out_json)?.into()))
    }
}

#[cfg(test)]
mod test {
    use std::collections::BTreeMap;

    use fluvio_smartmodule::Record;
    use serde_json::json;

    use super::*;

    #[test]
    fn test_query() {
        let creatures = json!([
            { "name": "Sammy", "type": "shark", "clams": 5 },
            { "name": "Bubbles", "type": "orca", "clams": 3 },
            { "name": "Splish", "type": "dolphin", "clams": 2 },
            { "name": "Splash", "type": "dolphin", "clams": 2 }
        ]);

        let recorddata = RecordData::from(creatures.to_string().as_bytes());
        let record = Record::new(recorddata);
        let sm_record = SmartModuleRecord::new(record, 0, 100);

        let mut params = BTreeMap::new();
        params.insert("filter".to_string(), ".[] | .name".to_string());
        init(params.into()).expect("failed");
        let (_key, value) = map(&sm_record).expect("failed");

        let out = serde_json::from_slice::<Value>(value.as_ref()).expect("failed");
        assert_eq!(out, json!(["Sammy", "Bubbles", "Splish", "Splash"]));
    }
}
