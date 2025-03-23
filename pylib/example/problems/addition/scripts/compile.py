import traopy_util as trau

meta = trau.ExecutionMetadata()

input_path = meta.get_submission_path()
output_path = meta.get_output_path()
language = meta.get_language()

trau.build(input_path, output_path, language)
