/// Represents a local variable in the current scope
#[derive(Debug, Clone)]
pub struct Local {
    name: String,
    depth: usize,
    initialized: bool,
}

/// Contains local variables
#[derive(Debug, Clone, Default)]
pub struct Locals {
    locals: Vec<Local>,
    depth: usize,
}

impl Locals {
    /// Find the index of a local variable by name.
    pub fn resolve_local(&self, name: &str) -> Option<usize> {
        for (i, local) in self.locals.iter().enumerate().rev() {
            if local.name == *name && local.initialized {
                return Some(i);
            }
        }
        None
    }

    pub fn check_local_on_depth(&self, name: &str, depth: usize) -> bool {
        if let Some(offset) = self.resolve_local(name) {
            return self.locals[offset].depth == depth;
        }
        false
    }

    pub fn check_local(&self, name: &str) -> bool {
        self.check_local_on_depth(name, self.depth)
    }

    pub fn add_local(&mut self, name: &str) -> Local {
        let local = Local {
            name: name.to_string(),
            depth: self.depth,
            initialized: false,
        };
        self.locals.push(local.clone());
        local
    }

    pub fn begin_scope(&mut self) {
        self.depth += 1;
    }

    pub fn end_scope(&mut self) -> usize {
        let locals_in_scope = self
            .locals
            .iter()
            .rev()
            .take_while(|local| local.depth == self.depth)
            .count();
        for _ in 0..locals_in_scope {
            self.locals.pop();
        }
        self.depth -= 1;
        locals_in_scope
    }

    pub fn initialize_last_local(&mut self) {
        self.locals.last_mut().unwrap().initialized = true;
    }

    pub fn depth(&self) -> usize {
        self.depth
    }

    pub fn last_index(&self) -> usize {
        &self.locals.len() - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_local() {
        let mut locals = Locals::default();
        locals.begin_scope();
        let local = locals.add_local("a");
        assert_eq!(local.depth, 1);

        locals.begin_scope();
        let local = locals.add_local("b");
        assert_eq!(local.depth, 2);

        locals.end_scope();
        let local = locals.add_local("c");
        assert_eq!(local.depth, 1);

        locals.end_scope();
    }

    #[test]
    fn begin_scope() {
        let mut locals = Locals::default();
        locals.begin_scope();
        assert_eq!(locals.depth, 1);

        locals.begin_scope();
        assert_eq!(locals.depth, 2);
    }

    #[test]
    fn end_scope() {
        let mut locals = Locals::default();
        locals.begin_scope(); // outer scope
        locals.begin_scope(); // inner scope
        locals.add_local("a");
        locals.add_local("b");
        let locals_in_scope = locals.end_scope();
        assert_eq!(locals_in_scope, 2, "inner scope had 2 variables");
        assert_eq!(locals.depth, 1, "inner scope ended");
        let locals_in_scope = locals.end_scope();
        assert_eq!(locals_in_scope, 0, "outer scope had no variables");
        assert_eq!(locals.depth, 0, "outer scope ended");
    }

    #[test]
    fn resolve_locals() {
        let mut locals = Locals::default();
        locals.begin_scope();
        locals.add_local("a");
        locals.initialize_last_local();
        locals.begin_scope();
        locals.add_local("b");
        locals.initialize_last_local();

        assert_eq!(locals.resolve_local("a"), Some(0));
        assert_eq!(locals.resolve_local("b"), Some(1));
        assert_eq!(locals.resolve_local("c"), None);

        locals.end_scope();
        assert_eq!(
            locals.resolve_local("a"),
            Some(0),
            "a should still be in scope"
        );
        assert_eq!(locals.resolve_local("b"), None, "b should not be in scope");
    }

    #[test]
    fn check_local_on_depth_level() {
        let mut locals = Locals::default();
        locals.begin_scope();
        locals.add_local("a");
        locals.initialize_last_local();
        locals.begin_scope();
        locals.add_local("b");
        locals.initialize_last_local();

        assert!(
            locals.check_local_on_depth("a", 1),
            "a should be on depth 1"
        );
        assert!(
            !locals.check_local_on_depth("a", 2),
            "a should not be on depth 2"
        );
        assert!(
            !locals.check_local_on_depth("b", 1),
            "b should not be on depth 1"
        );
        assert!(
            locals.check_local_on_depth("b", 2),
            "b should be on depth 2"
        );
        assert!(
            !locals.check_local_on_depth("c", 2),
            "c should not be on any level"
        );
    }
}
