void init(char* config);

char* generate_chunk_proof(const char* input, char* fork_name);

char* generate_batch_proof(const char* input, char* fork_name);

char* generate_bundle_proof(const char* input, char* fork_name);

void free_proof(char* proof);
