import os

output_file = os.getenv('output')
actual_output_file = os.getenv('actual')

if output_file and actual_output_file:
    with open(output_file, 'r', encoding='utf-8') as f:
        output_text = f.read()

    with open(actual_output_file, 'r', encoding='utf-8') as f:
        actual_output = f.read()

    if output_text == actual_output:
        print("Test Passed")
    else:
        print("Test Failed")
else:
    print("Input and output files not found")
