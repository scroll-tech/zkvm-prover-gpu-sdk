// initialize the library
void init(char* config);

// to generate proofs, proofs are encoded in json
char* generate_chunk_proof(const char* input, char* fork_name);

char* generate_batch_proof(const char* input, char* fork_name);

char* generate_bundle_proof(const char* input, char* fork_name);

void free_proof(char* proof);

// vks are encoded in base64
char* get_chunk_vk();

char* get_batch_vk();

char* get_bundle_vk();

void free_vk(char* vk);
