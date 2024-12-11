CREATE TABLE tarefa (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    descricao TEXT NOT NULL,
    data_inicio TEXT NOT NULL,
    data_fim TEXT,
    data_fim_user TEXT,
    id_meta INTEGER,
    FOREIGN KEY (id_meta) REFERENCES meta (id)
);