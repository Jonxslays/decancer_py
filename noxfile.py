import nox


@nox.session(reuse_venv=True)
def tests(session: nox.Session) -> None:
    session.run("poetry", "install", "-n", external=True)
    session.run("maturin", "develop")
    session.run("pytest", "--verbose")
