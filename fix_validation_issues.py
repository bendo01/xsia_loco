#!/usr/bin/env python3
import os
import re
import sys

def fix_controller_validation(file_path):
    """Fix validation issues in controller files"""
    try:
        with open(file_path, 'r') as f:
            content = f.read()
        
        original_content = content
        
        # Pattern 1: Remove the first validation block that checks payload.validate()
        pattern1 = r'(\s+)let mut validation_errors = ValidationErrors::new\(\);\s*\n\s+if let Err\(validation_errors\) = payload\.validate\(\) \{\s*\n\s+let error_json = format_validation_errors\(&validation_errors, "Validation Failed"\);\s*\n\s+return Ok\(\(\s*\n\s+StatusCode::UNPROCESSABLE_ENTITY, // Set status to 422\s*\n\s+Json\(error_json\),\s+// Return JSON-formatted errors\s*\n\s+\)\s*\n\s+\.into_response\(\)\);\s*\n\s+\}\s*\n'
        content = re.sub(pattern1, r'\1let mut validation_errors = ValidationErrors::new();\n', content, flags=re.MULTILINE)
        
        # Pattern 2: Remove the second validation call payload.validate()?;
        pattern2 = r'\s+// Validate the payload using validator\s*\n\s+payload\.validate\(\)\?;\s*\n'
        content = re.sub(pattern2, '\n', content, flags=re.MULTILINE)
        
        # Pattern 3: Remove standalone payload.validate()?; calls
        pattern3 = r'\s+payload\.validate\(\)\?;\s*\n'
        content = re.sub(pattern3, '', content, flags=re.MULTILINE)
        
        # Write back if changed
        if content != original_content:
            with open(file_path, 'w') as f:
                f.write(content)
            print(f"Fixed: {file_path}")
            return True
        else:
            print(f"No changes needed: {file_path}")
            return False
            
    except Exception as e:
        print(f"Error processing {file_path}: {e}")
        return False

def main():
    if len(sys.argv) < 2:
        print("Usage: python3 fix_validation_issues.py <file1> [file2] ...")
        sys.exit(1)
    
    fixed_count = 0
    for file_path in sys.argv[1:]:
        if fix_controller_validation(file_path):
            fixed_count += 1
    
    print(f"Fixed {fixed_count} files")

if __name__ == "__main__":
    main()
