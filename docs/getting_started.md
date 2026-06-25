# Getting Started with vscience

Welcome to vscience! This guide will help you get started with using vscience for your data science projects.

## Before You Begin
Before you start using vscience, make sure you have Python installed on your system. You can download Python from the official website: [https://www.python.org/downloads/](https://www.python.org/downloads/).

It is also recommended to install uv before using vscience, as it is used for managing virtual environments. You can install uv using pip:

```bash
pip install uv
```
or

```bash
python -m pip install uv
```
if pip is not available in your PATH.


## Installation
To install vscience, you can use pip:

```bash
pip install vscience
```

## Creating a New Project
To create a new data science project using vscience, you can use the following command:

```bash 
vsci init [project_name] [project_path]
```

**Arguments:**
- `[project_name]`: The name of your new project. If not specified, 'my_project' will be used as the default name.
- `[project_path]`: The path where you want to create the project. If not specified, the project will be created in the current directory.

When creating a new project, vscience will automatically set up a standardized project structure, including directories for code, data, and documentation. In addition, it will prompt you to choose from popular data science libraries (such as pandas, numpy, scikit-learn, etc.) to include in your project. This ensures that your project is set up with the necessary dependencies from the start.

## Activating the Virtual Environment
After creating a new project, you can activate the virtual environment for your project using the following command
```bash
    vsci activate [project_directory]
```

**Arguments:**
- `[project_directory]`: The path to your project directory. If not specified, the current directory will be used.

Starting the virtual environment will ensure that your project dependencies are isolated and well-organized, allowing you to work on your data science project without worrying about conflicts with other projects or system-wide packages. You may need to activate the virtual environment each time you start working on your project.

Now you can now start working on your project, running experiments, and documenting your progress. For more information on how to use vscience, please refer to the [features](features.md).