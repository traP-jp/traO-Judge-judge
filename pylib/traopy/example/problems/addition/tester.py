#!/usr/bin/env python3
import os
output_file = os.getenv('OUTPUT')
actual_output_file = os.getenv('SCRIPT')

if output_file and actual_output_file:
    with open(output_file, 'r', encoding='utf-8') as f:
        output_text = f.read()

    with open(actual_output_file, 'r', encoding='utf-8') as f:
        actual_output = f.read()

    STATUS = "AC" if output_text == actual_output else "WA"

    result_json = f'''{{
        "result":{{
            "Displayable":{{
                "status":"{STATUS}",
                "time":0.1,
                "memory":0.2,
                "score":100,
                "message":"Hello"
            }}
        }},
        "continue_status":"Continue"
    }}'''
    print(result_json)

else:
    print("OUTPUT or SRC not found")