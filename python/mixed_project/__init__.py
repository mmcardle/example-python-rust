from .mixed_project import run


def run_in_python(t):
	return run(t)

__doc__ = mixed_project.__doc__
if hasattr(mixed_project, "__all__"):
	__all__ = mixed_project.__all__