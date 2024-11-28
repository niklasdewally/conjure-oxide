searchState.loadedDescShard("conjure_core", 0, "Triggers a panic with a detailed bug report message, while …\nTop-level error types for Conjure-Oxide.\nThis module contains the rewrite rules for Conjure Oxides …\nA high-level API for interacting with constraints solvers.\n<strong>Supported by:</strong> Minion.\n<strong>Supported by:</strong> JsonInput, SAT.\nAn <code>Atom</code> is an indivisible expression, such as a literal or …\nDeclaration of an auxiliary variable.\nAn expression representing “A is valid as long as B is …\nRepresents a decision variable within a computational …\n<code>a / b = c</code> <strong>Supported by:</strong> Minion.\n<strong>Supported by:</strong> JsonInput.\nRepresents different types of expressions used to define …\n<strong>Supported by:</strong> JsonInput.\n<strong>Supported by:</strong> JsonInput.\n<strong>Supported by:</strong> Minion.\n<strong>Supported by:</strong> JsonInput.\nA literal value, equivalent to constants in Conjure.\n<strong>Supported by:</strong> JsonInput.\n<strong>Supported by:</strong> JsonInput.\n<strong>Supported by:</strong> JsonInput.\nBinary subtraction operator\n<code>a % b = c</code> <strong>Supported by:</strong> Minion.\nNegation: <code>-x</code> <strong>Supported by:</strong> JsonInput.\n<strong>Supported by:</strong> JsonInput.\n<strong>Supported by:</strong> JsonInput, SAT.\n<strong>Supported by:</strong> JsonInput, SAT.\n<strong>Supported by:</strong> Minion.\nDivision after preventing division by zero, usually with a …\nModulo after preventing mod 0, usually with a bubble\n<strong>Supported by:</strong> Minion, JsonInput.\n<strong>Supported by:</strong> Minion.\n<strong>Supported by:</strong> Minion.\nDivision with a possibly undefined value (division by 0) …\nModulo with a possibly undefined value (mod 0) <strong>Supported </strong>…\nw-literal(x,k) is SAT iff x == k, where x is a variable …\nReturn an unoptimised domain that is the result of …\nReturns the possible values of the expression, recursing …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nTrue if the expression is an associative and commutative …\nReturn a list of all possible i32 values in the domain if …\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nContains the error value\nContains the success value\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nRepresents a computational model containing variables, …\nExtends the models symbol table with the given symbol …\nReturns the argument unchanged.\nReturns an arbitrary variable name that is not in the …\nCalls <code>U::from(self)</code>.\nSearches recursively in <code>../tests/integration</code> folder for an …\nSearches for an <code>.essence</code> file at the given filepath, then …\nThe result of applying a rule to an expression. Contains …\nContains the error value\nContains the success value\nRepresents the result of applying a rule to an expression …\nRepresents errors that can occur during the model …\nA rule with a name, application function, and rule sets.\nA structure representing a set of rules with a name, …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nGet the dependencies of this rule set, evaluating them …\nGet a rule by name. Returns the rule with the given name …\nConvert a list of rule sets into a final map of rules to …\nGet a rule set by name. Returns the rule set with the …\nGet all rule sets Returns a <code>Vec</code> of static references to …\nGet all rule sets for a given solver family. Returns a <code>Vec</code> …\nReturns a copied <code>Vec</code> of all rules registered with the …\nGet the rules of this rule set, evaluating them lazily if …\nGet a final ordering of rules based on their priorities …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nThe name of the rule set.\nOrder of the RuleSet. Used to establish a consistent order …\nRepresents a reduction with no side effects on the model.\nThis procedural macro registers a decorated function with …\nThis procedural macro registers a rule set with the global …\nResolves the final set of rule sets to apply based on …\nRewrites the given model by applying a set of rules to all …\nThe solver families that this rule set applies to.\nGet the dependencies of this rule set, including itself\nRepresents a reduction that also modifies the symbol table.\nRepresents a reduction that also adds a top-level …\nSimplify an expression to a constant if possible Returns: …\nThe search was complete (i.e. the solver found all …\nThe search was incomplete (i.e. it was terminated before …\nReturned from SolverAdaptor when solving is successful.\nAn abstract representation of a constraints solver.\nA common interface for calling underlying solver APIs …\nThe type for user-defined callbacks for use with Solver.\nErrors returned by Solver on failure.\nAn iterator over the variants of SolverFamily\nSolver adaptors.\nAdds the solver adaptor name and family (if they exist) to …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nGet the solver family that this solver adaptor belongs to\nGets the name of the solver adaptor for pretty printing.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nModifying a model during search.\nRuns the solver on the given model.\nRuns the solver on the given model, allowing modification …\nStates of a <code>Solver</code>.\nA SolverAdaptor for interacting with the Kissat SAT solver.\nA SolverAdaptor for interacting with Minion.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nAn unspecified error has occurred.\nA ModelModifier provides an interface to modify a model …\nThe requested modification to the model has failed.\nA <code>ModelModifier</code> for a solver that does not support …\nThe desired operation is supported by this solver adaptor, …\nThe desired operation is not supported for this solver …\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nThe state returned by <code>Solver</code> if solving has not been …\nThe state returned by <code>Solver</code> if solving has been …\nCannot construct this from outside this module.\nCannot construct this from outside this module.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nExecution statistics.\nThe status of the search\nRepresents the statistical data collected during the model …\nWall time as measured by Conjure-Oxide (not the solver).\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nThe solver adaptor used for this run.\nThe solver family used for this run.")