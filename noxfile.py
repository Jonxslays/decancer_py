import nox


@nox.session(reuse_venv=True)
def tests(session: nox.Session) -> None:
    session.run_install(
        "uv",
        "sync",
        "--group",
        "dev",
        env={"UV_PROJECT_ENVIRONMENT": session.virtualenv.location},
    )
    session.run("maturin", "develop")
    session.run("pytest", "--verbose")
