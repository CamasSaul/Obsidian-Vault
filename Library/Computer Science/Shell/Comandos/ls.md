#comando #bash #shell #files
# Options

| Option | Long option      | Qué hace?                                                                                              |
| ------ | ---------------- | ------------------------------------------------------------------------------------------------------ |
| -a     | --all            | Muestra los archivos ocultos.                                                                          |
| -A     | --almost-all     | Muestra los archivos ocultos, excepto las referencias al directorio actual y el directorio padre.      |
| -d     | --directory      | Muestra únicamente información del directorio actual.                                                  |
| -F     | --classify       | Agrega un `/` al final del nombre de los directorios para indicar explícita mente que son directorios. |
| -h     | --human-readable | Muestra la medida de espacio en memoria en KB, MG, GB, etc.                                            |
| -l     |                  | Muestra la información de el contenido del directorio en formato largo (con más detalles).             |
| -r     | --reverse        | Muestra los resultados en orden inverso.                                                               |
| -S     |                  | Ordena los resultados por tamaño.                                                                      |
| -t     |                  | Ordena los resultados por fecha de modificación.                                                       |
# Long term output
```
$ ls -l
-rw-r--r-- 1 root root 453764 2017-04-03 11:05 oo-welcome.odt
```


| Permisos de acceso al archivo | Número de hard links | Usuario dueño | Nombre del grupo al que pertenece | Size (bytes) | Última modificación | Nombre del archivo |
| ----------------------------- | -------------------- | ------------- | --------------------------------- | ------------ | ------------------- | ------------------ |
| -rw-r--r--                    | 1                    | root          | root                              | 453764       | 2017-04-03 11:05    | oo-welcome.odt     |

# Commandos
- ### Listar archivos por peso (>) :
	`$ ls -lS`