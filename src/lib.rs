#![forbid(unsafe_code)]
/// Hermes Agent task execution model for SigmaOS agentic workflows.

#[derive(Debug, Clone, PartialEq)]
pub enum StepStatus {
    Pending,
    Running,
    Done,
    Error(String),
}

#[derive(Debug, Clone)]
pub struct AgentStep {
    pub thought: String,
    pub action: String,
    pub observation: Option<String>,
    pub status: StepStatus,
}

impl AgentStep {
    pub fn new(thought: &str, action: &str) -> Self {
        Self {
            thought: thought.into(),
            action: action.into(),
            observation: None,
            status: StepStatus::Pending,
        }
    }

    pub fn execute(&mut self, observation: &str) {
        self.observation = Some(observation.into());
        self.status = StepStatus::Done;
    }

    pub fn fail(&mut self, reason: &str) {
        self.status = StepStatus::Error(reason.into());
    }

    pub fn is_done(&self) -> bool {
        self.status == StepStatus::Done
    }
}

#[derive(Debug, Default)]
pub struct AgentTrace {
    pub steps: Vec<AgentStep>,
}

impl AgentTrace {
    pub fn push(&mut self, s: AgentStep) {
        self.steps.push(s);
    }
    pub fn done_count(&self) -> usize {
        self.steps.iter().filter(|s| s.is_done()).count()
    }
    pub fn has_errors(&self) -> bool {
        self.steps
            .iter()
            .any(|s| matches!(s.status, StepStatus::Error(_)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step_execute_sets_done_aaa() {
        let mut s = AgentStep::new("I need to check", "search(query)");
        s.execute("Found 5 results");
        assert!(s.is_done());
    }

    #[test]
    fn test_step_fail_sets_error_aaa() {
        let mut s = AgentStep::new("try action", "run(cmd)");
        s.fail("command not found");
        assert!(matches!(s.status, StepStatus::Error(_)));
    }

    #[test]
    fn test_trace_done_count_aaa() {
        let mut t = AgentTrace::default();
        let mut s1 = AgentStep::new("t1", "a1");
        s1.execute("obs");
        t.push(s1);
        t.push(AgentStep::new("t2", "a2"));
        assert_eq!(t.done_count(), 1);
    }

    #[test]
    fn test_trace_has_errors_aaa() {
        let mut t = AgentTrace::default();
        let mut s = AgentStep::new("t", "a");
        s.fail("oops");
        t.push(s);
        assert!(t.has_errors());
    }
}
