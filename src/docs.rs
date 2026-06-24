use serde_json::json;

pub fn api_docs() -> serde_json::Value {
    json!({
        "openapi": "3.0.0",
        "info": {
            "title": "Auto API",
            "version": "1.0.0"
        },
        "paths": {
            "/ai_agent": {
                "post": {
                    "summary": "Create AIAgent",
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": { "$ref": "#/components/schemas/AIAgent" }
                            }
                        }
                    },
                    "responses": {
                        "201": { "description": "Created" }
                    }
                }
            }
        },
        "components": {
            "schemas": {
                "AIAgent": {
                    "type": "object",
                    "properties": {
                        "id": { "type": "string" },
                        "name": { "type": "string" }
                    }
                }
            }
        }
    })
}