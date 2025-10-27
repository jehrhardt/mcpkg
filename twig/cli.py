import typer

app = typer.Typer(no_args_is_help=True)


@app.command()
def start() -> None:
    """Start the MCP server."""
    import twig.mcp
    import asyncio

    asyncio.run(twig.mcp.run())


@app.command()
def prompt() -> None:
    print("Hello")


def run() -> None:
    """Run the CLI application."""
    app()
