# Features

## Project Management

### **Create a new project**
```bash
    vsci init [project_name] [project_path]
```
**Arguments:**
- `[project_name]`: The name of your new project.
- `[project_path]`: The path where you want to create the project. If not specified, the project will be created in the current directory.

### **Add dependencies to a project**
```bash
    vsci add [dependency_name]
```
**Options:**
- `-d` `[dependency_name]`: The name of the dependency you want to add to your project.
- `-r` `[requirements_file]`: Specify a requirements file to add multiple dependencies at once.
- `-c` `[constraints_file]`: Specify a constraints file to add dependencies with specific version constraints.

### **Remove dependencies from a project**
```bash
    vsci remove <dependency_name>
```



