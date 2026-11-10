AI tasks:

[ ] clean up unused tool code, so we're sure we only inlcude the correct tools for genearting the waapi schemas and only the only the correct one for the api generation, we have multiple at the moment left over from older experiments. 

[ ] We want the waapi_clinet to always be the entry point for URI used from the generated functions and topics. We want the generated API to call into this waapi client too but with more type safe function calls. 

[ ] We need the function and topic URI be a enum so we can have typed URIs passed to the client rather than just raw string ref. We need to ensure that the user only calls the actual URIs used by wwise and to minimize errors.

[ ] We need to make sure we use the waapi_function_api.rs and waapi_topics_api.rs for all calls, we want to always use the str& that we define in those files. 

[ ] Define a set of WaapiError::Connection enums to be used instead of passing a string so we have a fixed set of errors for the connection

[ ] Make json macro wrapper for serde, in wappi client crate for easier use of raw calls. User should be able to not have to import in every app and just include waapi client

Tasks for me: 
[ ] Test out get and set WAAPI, to solidify API structure and flow for using the options and args before we proceed with setting up the full API 
