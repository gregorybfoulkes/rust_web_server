#!/bin/bash

curl -X POST http://localhost:3001/tasks \
-H "Content-Type: application/json" \
-d '{"title": "New Task", "description": "Description of the new task"}'
