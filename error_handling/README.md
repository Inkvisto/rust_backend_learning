
# Error handling tips

### Sources of [article](https://www.lpalmieri.com/posts/error-handling-rust/) & [video](https://www.youtube.com/watch?v=jpVzSse7oJ4&t=807s) 

1. Use enums when you expect the caller to react differently to the possible failure modes. Errors should be logged when they are handled.

2. Provide different error messages to users and operators. 
 - We expect errors to carry enough context about the failure to produce a report for an operator (e.g. the developer) that contains enough details to go and troubleshoot the issue.
 - `Debug` for operators & `Display` for users reports.


## Visualization of the proper structure of error handling

|              | Internal               | At the edge   |
| ------------ | ---------------------- |-------------- |
| Control Flow | Types, methods, fields | Status codes  |
| Reporting    | Logs/traces            | Response body |


3. Libraries
- Removing the boilerplate with `thiserror`
- Erase the type of the source error with `anyhow` 
- Use [`Context`](https://docs.rs/anyhow/latest/anyhow/trait.Context.html) as the error bubbles up through the call chain

4.  Must be avoided: 
- "Ball Of Mud" Error Enums( overdetailed errors )
- Single huge enum used as error type for several functions with different failure modes
- Leaking implementation details
- Repetition of error code

