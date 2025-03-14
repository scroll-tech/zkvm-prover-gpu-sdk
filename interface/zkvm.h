void init(char* config);

int generate_chunk_proof(char* proof, const char* input, char* fork_name);

int generate_batch_proof(char* proof, const char* input, char* fork_name);

int generate_bundle_proof(char* proof, const char* input, char* fork_name);
