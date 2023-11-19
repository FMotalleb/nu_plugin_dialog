use nu_plugin::{EvaluatedCall, LabeledError};
use nu_protocol::Value;

use crate::{prompt::UserPrompt, DialogPlugin};

impl DialogPlugin {
    pub(crate) fn confirm(
        &self,
        call: &EvaluatedCall,
        _input: &Value,
    ) -> Result<Value, LabeledError> {
        let prompt: String = call.req(0)?;
        let default_val: Option<bool> = call.get_flag("default")?;
        let span = call.head;
        let mut confirm = dialoguer::Confirm::with_theme(&*self.theme).with_prompt(prompt);

        if let Some(val) = default_val {
            confirm = confirm.default(val);
        }

        if call.has_flag("abortable") {
            let result = confirm.ask_opt(span)?;

            if let Some(val) = result {
                Ok(Value::bool(val, span))
            } else {
                Ok(Value::nothing(span))
            }
        } else {
            let result = confirm.ask(call.head)?;
            Ok(Value::bool(result, span))
        }
    }
}
