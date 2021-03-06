poetry update
poetry run pre-commit run --all-files
poetry run isort .
poetry run black .
poetry run mypy .
poetry run flake8 .
poetry run pytest --asyncio-mode=strict --verbose
