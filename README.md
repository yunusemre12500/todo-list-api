# TODO List API
This API created for practice.

## Usage
<details>
  <summary>Using Docker</summary>
  Run <code>docker compose up -d</code>
</details>
<details>
  <summary>Without Docker</summary>
  <ol>
    <li>Run <code>make build</code> to build the source code.</li>
    <li>Run <code>make install</code> to move compiler binary to <code>/usr/local/bin</code>.</li>
    <li>Execute binary <code>/usr/local/bin/todo-app-api</code>.</li>
  </ol>
</details>

## TODO List
- [x] Implement API Versioning
- [x] Database Connection
- [x] Add Dockerfile
- [x] Add docker-compose.yaml
- [ ] Support configuration using .yaml files
- [ ] Add Support for User Specific Todo Lists
- [ ] Add Support for Support User Specific TODO Lists
- [ ] Implement Authentication
- [ ] Implement Request Validation
- [ ] Implement Caching using Redis
- [ ] Implement Rate Limiting

## License
The [MIT](https://github.com/yunusemrealtiner1/todo-app/blob/main/LICENSE) License
