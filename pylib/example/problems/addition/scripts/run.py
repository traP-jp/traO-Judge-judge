import traopy_util as trau

meta = trau.ExecutionMetadata()

build_output_path_env = "BUILD_OUTPUT_PATH"
build_output_path = meta.get_env(build_output_path_env)
input_file_path_env = "INPUT_FILE_PATH"
input_file_path = meta.get_env(input_file_path_env)
expected_file_path_env = "EXPECTED_FILE_PATH"
expected_file_path = meta.get_env(expected_file_path_env)
language = meta.get_language()

start
output = trau.run(build_output_path, input_file_path, language)
exit_code = output.get_exit_code()
if exit_code != 0:
    
else:
    stdout = output.get_stdout()
    formatted_stdout = ' '.join(stdout.split())
    with open(expected_file_path, 'r') as f:
        expected = f.read()
        formatted_expected = ' '.join(expected.split())
        if formatted_stdout == formatted_expected:
            trau.jsonify_displayable_output(
                
            )
        else:
            print("Test failed")
            print(f"Expected: {formatted_expected}")
            print(f"Got: {formatted_stdout}")
