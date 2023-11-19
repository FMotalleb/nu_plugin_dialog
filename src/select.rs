use dialoguer::{FuzzySelect, Select};
use nu_plugin::{EvaluatedCall, LabeledError};
use nu_protocol::Value;

use crate::DialogPlugin;

impl DialogPlugin {
    pub(crate) fn select(
        &self,
        call: &EvaluatedCall,
        _input: &Value,
    ) -> Result<Value, LabeledError> {
        let options: Vec<String> = call.req(0)?;
        let span = call.head;
        let is_fuzzy = call.has_flag("fuzzy");

        match is_fuzzy {
            true => {
                let mut select = FuzzySelect::new().items(&options);
                if let Some(prompt) = call.get_flag::<String>("prompt")? {
                    select = select.with_prompt(prompt);
                }
                if let Some(def) = call.get_flag::<usize>("default")? {
                    select = select.default(def);
                }
                let result = select.interact_opt();
                match result {
                    Ok(result) => {
                        if let Some(index) = result {
                            if let Some(result) = options.get(index) {
                                return Ok(Value::string(result, span));
                            }
                            return Err(LabeledError {
                                label: "Selection out of range".to_string(),
                                msg: "User selected a value that was out of range".to_string(),
                                span: None,
                            });
                        } else {
                            return Ok(Value::nothing(span));
                        }
                    }
                    _ => {
                        return Err(LabeledError {
                            label: "fuck".to_string(),
                            msg: "fuck".to_string(),
                            span: None,
                        })
                    }
                }
            }
            false => {
                let mut select = Select::new().items(&options);
                if let Some(prompt) = call.get_flag::<String>("prompt")? {
                    select = select.with_prompt(prompt);
                }
                if let Some(def) = call.get_flag::<usize>("default")? {
                    select = select.default(def);
                }
                let result = select.interact_opt();
                match result {
                    Ok(result) => {
                        if let Some(index) = result {
                            if let Some(result) = options.get(index) {
                                return Ok(Value::string(result, span));
                            }
                            return Err(LabeledError {
                                label: "fuck".to_string(),
                                msg: "fuck".to_string(),
                                span: None,
                            });
                        } else {
                            return Ok(Value::nothing(span));
                        }
                    }
                    _ => {
                        return Err(LabeledError {
                            label: "fuck".to_string(),
                            msg: "fuck".to_string(),
                            span: None,
                        })
                    }
                }
            }
        };
    }
}
