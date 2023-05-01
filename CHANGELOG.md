## Unreleased (02fb6f2..dccc835)
#### Bug Fixes
- **(deps)** update rust crate clap to 4.2.5 - (93a87a0) - renovate[bot]
- **(doc)** fix project name in doc configuration - (e0888ee) - Dmytro Kovalchuk
- pop value of expression statements - (66f3914) - Dmytro Kovalchuk
- remove complex array assignments - (439712b) - Dmytro Kovalchuk
#### Continuous Integration
- cache dependencies for testing and linting - (60b95f3) - Dmytro Kovalchuk
- avoid tarpaulin compilation - (2cae88d) - Dmytro Kovalchuk
- migrate from usnupported rs-actions - (eb5e4f2) - Dmytro Kovalchuk
- ignore changes in docs - (3c707a5) - Dmytro Kovalchuk
- add test coverage measurements - (824dc51) - Dmytro Kovalchuk
- setup build pipeline for the project - (27ab20c) - Dmytro Kovalchuk
- scaffold and action to publish docs - (3a4026b) - Dmytro Kovalchuk
#### Documentation
- **(build)** rename build badges - (63df92a) - Dmytro Kovalchuk
- **(cli)** update about section of the cli - (ffbab44) - Dmytro Kovalchuk
- **(cli)** document CLI usage - (6c1fd3b) - Dmytro Kovalchuk
- **(vm)** add vm instructions reference - (dccc835) - Dmytro Kovalchuk
- add note on implementation language - (f920c6f) - Dmytro Kovalchuk
- add high-level implementation overview - (943224d) - Dmytro Kovalchuk
- add brainterpreter part description - (64df3a4) - Dmytro Kovalchuk
- add readme on documentation build - (28ab966) - Dmytro Kovalchuk
- fix text alignment in README - (1919b74) - Dmytro Kovalchuk
- add build status badges - (f59413b) - Dmytro Kovalchuk
- rename the language - (26c5c66) - Dmytro Kovalchuk
- add reference to project pages - (fbe8e15) - Dmytro Kovalchuk
- add znai-based site - (20a5941) - Dmytro Kovalchuk
- add README file - (c58819d) - Dmytro Kovalchuk
#### Features
- **(cli)** add verbose flag to interpreter cli - (e91dd59) - Dmytro Kovalchuk
- **(cli)** add cli to run interpreter - (c1787c2) - Dmytro Kovalchuk
#### Miscellaneous Chores
- **(ci)** add renovate bot configuration - (c10d765) - renovate[bot]
- **(deps)** update actions/setup-java action to v3 - (6f7f6f3) - renovate[bot]
- **(deps)** update gabrielbb/xvfb-action action to v1.6 - (1ad7ab9) - renovate[bot]
- **(deps)** update actions/checkout action to v3 - (aa8e734) - renovate[bot]
- exclude env_logger from lib dependencies - (695527a) - Dmytro Kovalchuk
- bump version - (02fb6f2) - Dmytro Kovalchuk
#### Performance Improvements
- use shared references for arrays - (e47a3a7) - Dmytro Kovalchuk
- add benchmarks - (836ae6d) - Dmytro Kovalchuk
#### Refactoring
- chage source files extensions - (9c74876) - Dmytro Kovalchuk
- rename the package - (22daab4) - Dmytro Kovalchuk
- encapsulate native function behaviour - (cf3bc53) - Dmytro Kovalchuk
- remove unused ast elements - (e87efdb) - Dmytro Kovalchuk
- use chunk reference instead of copying - (77fc7cd) - Dmytro Kovalchuk
- move jump calculation to call frame - (b73c731) - Dmytro Kovalchuk
- move call frame to separate file - (e8b2978) - Dmytro Kovalchuk
- remove unnecessary error type - (9b8120f) - Dmytro Kovalchuk
- start moving vm code to separate locations - (476f244) - Dmytro Kovalchuk
- move vm back to module core - (aad4c18) - Dmytro Kovalchuk
- extract expression parsing methods - (8aa59fd) - Dmytro Kovalchuk
- use single statement for function - (447f6fe) - Dmytro Kovalchuk
#### Tests
- add tests for parser advance - (1077014) - Dmytro Kovalchuk
- increase coverage - (072a27e) - Dmytro Kovalchuk
- add expression parser tests - (7c092c1) - Dmytro Kovalchuk
- test statement parser - (0b6af50) - Dmytro Kovalchuk

- - -
## v0.1.1 - 2023-05-01
#### Bug Fixes
- **(deps)** update rust crate clap to 4.2.5 - (93a87a0) - renovate[bot]
- **(doc)** fix project name in doc configuration - (e0888ee) - Dmytro Kovalchuk
- **(functions)** stop leaking value stack on function declaration - (44c1e60) - Dmytro Kovalchuk
- **(globals)** stop leaking value stack on variable declaration - (1223c41) - Dmytro Kovalchuk
- pop value of expression statements - (66f3914) - Dmytro Kovalchuk
- remove complex array assignments - (439712b) - Dmytro Kovalchuk
#### Continuous Integration
- cache dependencies for testing and linting - (60b95f3) - Dmytro Kovalchuk
- avoid tarpaulin compilation - (2cae88d) - Dmytro Kovalchuk
- migrate from usnupported rs-actions - (eb5e4f2) - Dmytro Kovalchuk
- ignore changes in docs - (3c707a5) - Dmytro Kovalchuk
- add test coverage measurements - (824dc51) - Dmytro Kovalchuk
- setup build pipeline for the project - (27ab20c) - Dmytro Kovalchuk
- scaffold and action to publish docs - (3a4026b) - Dmytro Kovalchuk
#### Documentation
- **(build)** rename build badges - (63df92a) - Dmytro Kovalchuk
- **(cli)** update about section of the cli - (ffbab44) - Dmytro Kovalchuk
- **(cli)** document CLI usage - (6c1fd3b) - Dmytro Kovalchuk
- **(vm)** add vm instructions reference - (dccc835) - Dmytro Kovalchuk
- add note on implementation language - (f920c6f) - Dmytro Kovalchuk
- add high-level implementation overview - (943224d) - Dmytro Kovalchuk
- add brainterpreter part description - (64df3a4) - Dmytro Kovalchuk
- add readme on documentation build - (28ab966) - Dmytro Kovalchuk
- fix text alignment in README - (1919b74) - Dmytro Kovalchuk
- add build status badges - (f59413b) - Dmytro Kovalchuk
- rename the language - (26c5c66) - Dmytro Kovalchuk
- add reference to project pages - (fbe8e15) - Dmytro Kovalchuk
- add znai-based site - (20a5941) - Dmytro Kovalchuk
- add README file - (c58819d) - Dmytro Kovalchuk
#### Features
- **(cli)** add verbose flag to interpreter cli - (e91dd59) - Dmytro Kovalchuk
- **(cli)** add cli to run interpreter - (c1787c2) - Dmytro Kovalchuk
#### Miscellaneous Chores
- **(ci)** add renovate bot configuration - (c10d765) - renovate[bot]
- **(deps)** update actions/setup-java action to v3 - (6f7f6f3) - renovate[bot]
- **(deps)** update gabrielbb/xvfb-action action to v1.6 - (1ad7ab9) - renovate[bot]
- **(deps)** update actions/checkout action to v3 - (aa8e734) - renovate[bot]
- **(release)** bump package version - (dba8ca8) - Dmytro Kovalchuk
- use cog to generate changelog - (971fdf2) - Dmytro Kovalchuk
- exclude env_logger from lib dependencies - (695527a) - Dmytro Kovalchuk
- bump version - (02fb6f2) - Dmytro Kovalchuk
#### Performance Improvements
- use shared references for arrays - (e47a3a7) - Dmytro Kovalchuk
- add benchmarks - (836ae6d) - Dmytro Kovalchuk
#### Refactoring
- chage source files extensions - (9c74876) - Dmytro Kovalchuk
- rename the package - (22daab4) - Dmytro Kovalchuk
- encapsulate native function behaviour - (cf3bc53) - Dmytro Kovalchuk
- remove unused ast elements - (e87efdb) - Dmytro Kovalchuk
- use chunk reference instead of copying - (77fc7cd) - Dmytro Kovalchuk
- move jump calculation to call frame - (b73c731) - Dmytro Kovalchuk
- move call frame to separate file - (e8b2978) - Dmytro Kovalchuk
- remove unnecessary error type - (9b8120f) - Dmytro Kovalchuk
- start moving vm code to separate locations - (476f244) - Dmytro Kovalchuk
- move vm back to module core - (aad4c18) - Dmytro Kovalchuk
- extract expression parsing methods - (8aa59fd) - Dmytro Kovalchuk
- use single statement for function - (447f6fe) - Dmytro Kovalchuk
#### Tests
- add tests for parser advance - (1077014) - Dmytro Kovalchuk
- increase coverage - (072a27e) - Dmytro Kovalchuk
- add expression parser tests - (7c092c1) - Dmytro Kovalchuk
- test statement parser - (0b6af50) - Dmytro Kovalchuk

- - -


## v0.1.0 - 2023-05-01
#### Bug Fixes
- pop value on variable assignment - (efd38cf) - Dmytro Kovalchuk
- replace offset value in stack - (fd4aa1c) - Dmytro Kovalchuk
#### Features
- **(tracing)** trace after instruction - (3f71716) - Dmytro Kovalchuk
- reduce memory footprint of the example - (5f7a9f7) - Dmytro Kovalchuk
- run brainfuck interpreter - (50ae03b) - Dmytro Kovalchuk
- support arrays - (cc8097b) - Dmytro Kovalchuk
- convert to string - (5afce2a) - Dmytro Kovalchuk
- base support of native functions - (8b0e0a0) - Dmytro Kovalchuk
- write string character by index - (38ea9a1) - Dmytro Kovalchuk
- support function arguments - (cf00b75) - Dmytro Kovalchuk
- access characters in array-like manner - (cb6bbcf) - Dmytro Kovalchuk
- initial support of strings - (0812588) - Dmytro Kovalchuk
- return values from functions - (27a953b) - Dmytro Kovalchuk
- call global functions - (5cc18d6) - Dmytro Kovalchuk
- declare global functions - (67c4c6b) - Dmytro Kovalchuk
- support while loops - (7c29572) - Dmytro Kovalchuk
- support if-else statements - (6993b73) - Dmytro Kovalchuk
- support simple if statements - (75863d4) - Dmytro Kovalchuk
- initialize local variables from upper scopes with shadowing - (c6a9e60) - Dmytro Kovalchuk
- add compilation errors - (b1c7986) - Dmytro Kovalchuk
- support local variables - (04f0b0e) - Dmytro Kovalchuk
- support unary negation and boolean literals - (daae2b1) - Dmytro Kovalchuk
- support comparisons - (c3b01fb) - Dmytro Kovalchuk
- initialize and read global variables - (af334f8) - Dmytro Kovalchuk
- declare global variables - (836543e) - Dmytro Kovalchuk
- compile multiple statements - (4e60d5a) - Dmytro Kovalchuk
- support print statements - (04d8c95) - Dmytro Kovalchuk
- support comment - (f42a95f) - Dmytro Kovalchuk
- track source positions in lexer and parser - (739ac6d) - Dmytro Kovalchuk
- add interpreter binary - (43ed692) - Dmytro Kovalchuk
- add floating point number literals - (b3c6a74) - Dmytro Kovalchuk
- group expressions using parentheses - (7c7aa57) - Dmytro Kovalchuk
- implement unary minus - (30bf787) - Dmytro Kovalchuk
- implement multiplication and division - (4031555) - Dmytro Kovalchuk
- parse operations of the same binding power - (e1a07a1) - Dmytro Kovalchuk
- temporarily add `ret` instruction to expressions - (ac4312f) - Dmytro Kovalchuk
- parse addition operation - (42fdb3a) - Dmytro Kovalchuk
- compile simple arithmetic expression - (adf6a39) - Dmytro Kovalchuk
- start lexer implementation - (662ee80) - Dmytro Kovalchuk
- print last stack value on return - (e169c19) - Dmytro Kovalchuk
- trace program execution - (bd9a6b1) - Dmytro Kovalchuk
- add error reporting - (323eaf8) - Dmytro Kovalchuk
#### Miscellaneous Chores
- add crate attributes - (a38f9df) - Dmytro Kovalchuk
- address linter messages - (e5c10d1) - Dmytro Kovalchuk
- ignore IDE files - (602271c) - Dmytro Kovalchuk
#### Refactoring
- move array access logic to types - (6dd37a9) - Dmytro Kovalchuk
- remove unused statements - (0a51b1c) - Dmytro Kovalchuk
- use expression for variable assignment - (3935495) - Dmytro Kovalchuk
- extract locals to own file - (354cb55) - Dmytro Kovalchuk
- get source position on demand - (da6768c) - Dmytro Kovalchuk
- improve parser error reporting - (553dd6e) - Dmytro Kovalchuk
- add precedence enum - (24e4fb3) - Dmytro Kovalchuk
- rename array access operation - (c505543) - Dmytro Kovalchuk
- split parser to dedicated modules - (e9f8d4e) - Dmytro Kovalchuk
- split parser to dedicated modules - (ce77bc9) - Dmytro Kovalchuk
- use call frame base to place variables - (61c3ced) - Dmytro Kovalchuk
- rename vm module to avoid clippy violations - (728b96a) - Dmytro Kovalchuk
- use call frames when running code - (a7ddc75) - Dmytro Kovalchuk
- add dedicated method for common jumps - (cc0bcde) - Dmytro Kovalchuk
- simplify offset jump calculation - (ff0ba1e) - Dmytro Kovalchuk
- move vm to dedicated file - (4be917f) - Dmytro Kovalchuk
- move opcodes to a vm mod - (f68aa8a) - Dmytro Kovalchuk
- process unsupported infix operations - (a102ce6) - Dmytro Kovalchuk
- remove unused field from parser - (61102a5) - Dmytro Kovalchuk
- add convenience methods to construct AST for tests - (93756c2) - Dmytro Kovalchuk
- parametrize binary operations in AST - (60970b1) - Dmytro Kovalchuk
- add utility function to interpret source - (42622db) - Dmytro Kovalchuk
- use dedicated method to parse digits - (40f1bdb) - Dmytro Kovalchuk
#### Style
- move implementation blocks for convenience - (50fec87) - Dmytro Kovalchuk
#### Tests
- **(example)** fully process addition expression - (cc62f53) - Dmytro Kovalchuk
- add test to keep brainfuck implementation intact - (80efb3e) - Dmytro Kovalchuk
- automated output testing for integration tests - (411cffa) - Dmytro Kovalchuk
- test same priority expression - (50f623a) - Dmytro Kovalchuk


