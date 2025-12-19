"""
Simple post processing script to fix the output of the generated models.
"""

import glob
from pathlib import Path
import os
from textwrap import dedent
OUTPUT_FOLDER = Path("src/models/")

class PostProcessor:
    """
    Class to handle post processing of generated OpenAPI models.
    """

    def __init__(self):
        """
        Initialize the post processor.
        """
        self.files = OUTPUT_FOLDER.glob("*.rs")
        self.files = list(self.files)
        print(f"Found {len(self.files)} files to process.")


    def work(self):
        """
        Main work function to process files.
        """
        for file_path in self.files:
            self.process_file(file_path)
            self.rename_file_if_needed(file_path)
        self.update_mod_file()

    def process_file(self, file_path: Path):
        """
        Process a single file to fix issues.
        """
        content = Path(file_path).read_text()
        original_content = content

        # We replace all occurences of 200Response
        content = content.replace("200Response", "")
        # we also replace any instances of " Rest" with " "
        content = content.replace("Rest", "")
        if content != original_content:
            file_path.write_text(content)

    def rename_file_if_needed(self, file_path: Path):
        """
        Rename file if it contains 'rest_' in its file name.
        """
        new_name = file_path.name \
            .replace("_200_response", "") \
            .replace("__", "_")
        if new_name.startswith("rest_"):
            new_name = new_name[5:]
        new_path = file_path.parent / new_name
        if new_path != file_path and new_path.exists():
            raise FileExistsError(f"Cannot rename {file_path} to {new_path}, target already exists.")
        file_path.rename(new_path)
        self.rebuild_mod_file()


    def rebuild_mod_file(self):
        """
        Rebuild the mod.rs file from scratch.
        """
        header = dedent("""
                        #![allow(clippy::all)]
                        #![allow(unused_imports)]
                        #![allow(dead_code)]
                        #![allow(non_camel_case_types)]
                        #![allow(clippy::upper_case_acronyms)]
                        """).strip() + "\n\n"
        mod_file_path = OUTPUT_FOLDER / "mod.rs"
        lines = [header]
        for file_path in sorted(OUTPUT_FOLDER.glob("*.rs")):
            if file_path.name == "mod.rs":
                continue
            name = file_path.stem
            camel = ''.join(part.capitalize() for part in name.split('_'))
            lines.append(f"pub mod {name};")
            lines.append(f"pub use {name}::{camel};")
        mod_file_path.write_text('\n'.join(lines))

    def update_mod_file(self):
        """
        Update the mod.rs file to reflect any renamed files.
        """
        mod_file_path = OUTPUT_FOLDER / "mod.rs"
        if not mod_file_path.exists():
            return

        content = mod_file_path.read_text()
        original_content = content

        # Replace any instances of 'rest_' in mod file
        content = content.replace("\nrest_", "\n") \
            .replace("_200_response", "") \
            .replace("__", "_")
        if content != original_content:
            print("Updating mod.rs file.")
            mod_file_path.write_text(content)

if __name__ == "__main__":
    processor = PostProcessor()
    processor.work()
        




