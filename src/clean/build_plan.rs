use super::types::Step;
use crate::rules::Clean;
use crate::scan::Item;

/// The exact steps for the selected items. A command shared by several items runs once.
pub fn build_plan(items: &[Item]) -> Vec<Step> {
    let mut steps = Vec::new();
    for item in items.iter().filter(|i| i.selected && i.keep_reason.is_none()) {
        match item.rule.clean {
            Clean::RemoveDirs => {
                steps.extend(item.paths.iter().map(|p| Step::Remove {
                    path: p.clone(),
                    rule: item.rule,
                }));
            }
            Clean::Run(cmd) => {
                if !steps.iter().any(|s| matches!(s, Step::Run(c) if std::ptr::eq(*c, cmd))) {
                    steps.push(Step::Run(cmd));
                }
            }
        }
    }
    steps
}
