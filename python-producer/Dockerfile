FROM python:3.9

WORKDIR /app

COPY . .

RUN pip install poetry
RUN poetry config virtualenvs.create false
RUN poetry install --no-dev

CMD ["python", "src/main.py"]
