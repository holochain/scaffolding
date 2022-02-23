import { ScFile, ScNodeType } from '@source-craft/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const customElementsJson = ({moduleNameSnakeCase, moduleNamePluralTitleCase, _kebab, kebabPlural_, kebabSingular_, moduleNameTitleCase, moduleNamePlural, moduleName}: {moduleNameSnakeCase: string; moduleNamePluralTitleCase: string; _kebab: string; kebabPlural_: string; kebabSingular_: string; moduleNameTitleCase: string; moduleNamePlural: string; moduleName: string;}): ScFile => ({
  type: ScNodeType.File,
  content: `{
  "schemaVersion": "1.0.0",
  "readme": "",
  "modules": [
    {
      "kind": "javascript-module",
      "path": "src/config.ts",
      "declarations": [
        {
          "kind": "variable",
          "name": "defaultConfig",
          "type": {
            "text": "${moduleNamePluralTitleCase}Config"
          },
          "default": "{\\n  zomeName: '${moduleNamePlural}',\\n  avatarMode: 'avatar',\\n  additionalFields: [],\\n  minNicknameLength: 3,\\n}"
        }
      ],
      "exports": [
        {
          "kind": "js",
          "name": "defaultConfig",
          "declaration": {
            "name": "defaultConfig",
            "module": "src/config.ts"
          }
        }
      ]
    },
    {
      "kind": "javascript-module",
      "path": "src/context.ts",
      "declarations": [
        {
          "kind": "variable",
          "name": "${moduleNamePlural}StoreContext",
          "type": {
            "text": "Context<${moduleNamePluralTitleCase}Store>"
          }
        }
      ],
      "exports": [
        {
          "kind": "js",
          "name": "${moduleNamePlural}StoreContext",
          "declaration": {
            "name": "${moduleNamePlural}StoreContext",
            "module": "src/context.ts"
          }
        }
      ]
    },
    {
      "kind": "javascript-module",
      "path": "src/index.ts",
      "declarations": [],
      "exports": [
        {
          "kind": "js",
          "name": "*",
          "declaration": {
            "name": "*",
            "package": "./types"
          }
        },
        {
          "kind": "js",
          "name": "*",
          "declaration": {
            "name": "*",
            "package": "./context"
          }
        },
        {
          "kind": "js",
          "name": "*",
          "declaration": {
            "name": "*",
            "package": "./elements/create${_kebab}"
          }
        },
        {
          "kind": "js",
          "name": "*",
          "declaration": {
            "name": "*",
            "package": "./elements/update${_kebab}"
          }
        },
        {
          "kind": "js",
          "name": "*",
          "declaration": {
            "name": "*",
            "package": "./elements/my${_kebab}"
          }
        },
        {
          "kind": "js",
          "name": "*",
          "declaration": {
            "name": "*",
            "package": "./elements/search-agent"
          }
        },
        {
          "kind": "js",
          "name": "*",
          "declaration": {
            "name": "*",
            "package": "./elements/${kebabSingular_}prompt"
          }
        },
        {
          "kind": "js",
          "name": "*",
          "declaration": {
            "name": "*",
            "package": "./elements/list${_kebab}s"
          }
        },
        {
          "kind": "js",
          "name": "*",
          "declaration": {
            "name": "*",
            "package": "./elements/agent-avatar"
          }
        },
        {
          "kind": "js",
          "name": "*",
          "declaration": {
            "name": "*",
            "package": "./elements/holo-identicon"
          }
        },
        {
          "kind": "js",
          "name": "*",
          "declaration": {
            "name": "*",
            "package": "./elements/${kebabSingular_}detail"
          }
        },
        {
          "kind": "js",
          "name": "*",
          "declaration": {
            "name": "*",
            "package": "./${kebabPlural_}service"
          }
        },
        {
          "kind": "js",
          "name": "*",
          "declaration": {
            "name": "*",
            "package": "./${kebabPlural_}store"
          }
        },
        {
          "kind": "js",
          "name": "*",
          "declaration": {
            "name": "*",
            "package": "./config"
          }
        }
      ]
    },
    {
      "kind": "javascript-module",
      "path": "src/mocks.ts",
      "declarations": [
        {
          "kind": "class",
          "description": "",
          "name": "${moduleNamePluralTitleCase}ZomeMock",
          "members": [
            {
              "kind": "field",
              "name": "cellId",
              "type": {
                "text": "CellId"
              }
            },
            {
              "kind": "field",
              "name": "myPubKeyB64"
            },
            {
              "kind": "method",
              "name": "create${moduleNameSnakeCase}",
              "parameters": [
                {
                  "name": "{ nickname }",
                  "type": {
                    "text": "{ nickname: string }"
                  }
                }
              ]
            },
            {
              "kind": "method",
              "name": "search${moduleNameSnakeCase}s",
              "parameters": [
                {
                  "name": "{ nicknamePrefix }",
                  "type": {
                    "text": "{ nicknamePrefix: string }"
                  }
                }
              ]
            },
            {
              "kind": "method",
              "name": "get_my${moduleNameSnakeCase}"
            },
            {
              "kind": "method",
              "name": "get_agent${moduleNameSnakeCase}",
              "parameters": [
                {
                  "name": "agent_address",
                  "type": {
                    "text": "AgentPubKeyB64"
                  }
                }
              ]
            },
            {
              "kind": "method",
              "name": "get_all${moduleNameSnakeCase}s"
            },
            {
              "kind": "method",
              "name": "findAgent",
              "parameters": [
                {
                  "name": "agent_address",
                  "type": {
                    "text": "AgentPubKeyB64"
                  }
                }
              ]
            },
            {
              "kind": "method",
              "name": "callZome",
              "return": {
                "type": {
                  "text": "Promise<any>"
                }
              },
              "parameters": [
                {
                  "name": "zomeName",
                  "type": {
                    "text": "string"
                  }
                },
                {
                  "name": "fnName",
                  "type": {
                    "text": "string"
                  }
                },
                {
                  "name": "payload",
                  "type": {
                    "text": "any"
                  }
                },
                {
                  "name": "timeout",
                  "optional": true,
                  "type": {
                    "text": "number"
                  }
                }
              ]
            },
            {
              "kind": "method",
              "name": "addSignalHandler",
              "return": {
                "type": {
                  "text": "{ unsubscribe: () => void }"
                }
              },
              "parameters": [
                {
                  "name": "signalHandler",
                  "type": {
                    "text": "AppSignalCb"
                  }
                }
              ]
            }
          ],
          "superclass": {
            "name": "CellClient",
            "package": "@holochain-open-dev/cell-client"
          }
        }
      ],
      "exports": [
        {
          "kind": "js",
          "name": "${moduleNamePluralTitleCase}ZomeMock",
          "declaration": {
            "name": "${moduleNamePluralTitleCase}ZomeMock",
            "module": "src/mocks.ts"
          }
        }
      ]
    },
    {
      "kind": "javascript-module",
      "path": "src/${kebabPlural_}service.ts",
      "declarations": [
        {
          "kind": "class",
          "description": "",
          "name": "${moduleNamePluralTitleCase}Service",
          "members": [
            {
              "kind": "method",
              "name": "getMy${moduleNameTitleCase}",
              "return": {
                "type": {
                  "text": ""
                }
              },
              "description": "Get my ${moduleName}, if it has been created"
            },
            {
              "kind": "method",
              "name": "getAgent${moduleNameTitleCase}",
              "return": {
                "type": {
                  "text": ""
                }
              },
              "parameters": [
                {
                  "name": "agentPubKey",
                  "type": {
                    "text": "AgentPubKeyB64"
                  },
                  "description": "the agent to get the ${moduleName} for"
                }
              ],
              "description": "Get the ${moduleName} for the given agent, if they have created it"
            },
            {
              "kind": "method",
              "name": "getAgents${moduleNamePluralTitleCase}",
              "return": {
                "type": {
                  "text": ""
                }
              },
              "parameters": [
                {
                  "name": "agentPubKeys",
                  "type": {
                    "text": "AgentPubKeyB64[]"
                  },
                  "description": "the agents to get the ${moduleName} for"
                }
              ],
              "description": "Get the ${moduleNamePlural} for the given agent"
            },
            {
              "kind": "method",
              "name": "search${moduleNamePluralTitleCase}",
              "return": {
                "type": {
                  "text": ""
                }
              },
              "parameters": [
                {
                  "name": "nicknamePrefix",
                  "type": {
                    "text": "string"
                  },
                  "description": "must be of at least 3 characters"
                }
              ],
              "description": "Search ${moduleNamePlural} that start with nicknamePrefix"
            },
            {
              "kind": "method",
              "name": "getAll${moduleNamePluralTitleCase}",
              "return": {
                "type": {
                  "text": ""
                }
              },
              "description": "Get the ${moduleNamePlural} for all the agents in the DHT"
            },
            {
              "kind": "method",
              "name": "create${moduleNameTitleCase}",
              "return": {
                "type": {
                  "text": ""
                }
              },
              "parameters": [
                {
                  "name": "${moduleName}",
                  "type": {
                    "text": "${moduleNameTitleCase}"
                  },
                  "description": "the ${moduleName} to create"
                }
              ],
              "description": "Create my ${moduleName}"
            },
            {
              "kind": "method",
              "name": "update${moduleNameTitleCase}",
              "return": {
                "type": {
                  "text": ""
                }
              },
              "parameters": [
                {
                  "name": "${moduleName}",
                  "type": {
                    "text": "${moduleNameTitleCase}"
                  },
                  "description": "the ${moduleName} to create"
                }
              ],
              "description": "Update my ${moduleName}"
            },
            {
              "kind": "method",
              "name": "callZome",
              "privacy": "private",
              "parameters": [
                {
                  "name": "fn_name",
                  "type": {
                    "text": "string"
                  }
                },
                {
                  "name": "payload",
                  "type": {
                    "text": "any"
                  }
                }
              ]
            }
          ]
        }
      ],
      "exports": [
        {
          "kind": "js",
          "name": "${moduleNamePluralTitleCase}Service",
          "declaration": {
            "name": "${moduleNamePluralTitleCase}Service",
            "module": "src/${kebabPlural_}service.ts"
          }
        }
      ]
    },
    {
      "kind": "javascript-module",
      "path": "src/${kebabPlural_}store.ts",
      "declarations": [
        {
          "kind": "class",
          "description": "",
          "name": "${moduleNamePluralTitleCase}Store",
          "members": [
            {
              "kind": "field",
              "name": "_service",
              "type": {
                "text": "${moduleNamePluralTitleCase}Service"
              },
              "privacy": "private",
              "description": "Private",
              "default": "new ${moduleNamePluralTitleCase}Service(cellClient, this.config.zomeName)"
            },
            {
              "kind": "field",
              "name": "_known${moduleNamePluralTitleCase}Store",
              "type": {
                "text": "Writable<Dictionary<${moduleNameTitleCase}>>"
              },
              "privacy": "private"
            },
            {
              "kind": "field",
              "name": "myAgentPubKey",
              "type": {
                "text": "AgentPubKeyB64"
              },
              "privacy": "public",
              "description": "Static info"
            },
            {
              "kind": "field",
              "name": "known${moduleNamePluralTitleCase}",
              "type": {
                "text": "Readable<Dictionary<${moduleNameTitleCase}>>"
              },
              "privacy": "public",
              "description": "Readable stores"
            },
            {
              "kind": "field",
              "name": "my${moduleNameTitleCase}",
              "type": {
                "text": "Readable<${moduleNameTitleCase}>"
              },
              "privacy": "public"
            },
            {
              "kind": "method",
              "name": "${moduleName}Of",
              "return": {
                "type": {
                  "text": "Readable<${moduleNameTitleCase}>"
                }
              },
              "parameters": [
                {
                  "name": "agentPubKey",
                  "type": {
                    "text": "AgentPubKeyB64"
                  }
                }
              ]
            },
            {
              "kind": "field",
              "name": "config",
              "type": {
                "text": "${moduleNamePluralTitleCase}Config"
              }
            },
            {
              "kind": "method",
              "name": "fetchAll${moduleNamePluralTitleCase}",
              "return": {
                "type": {
                  "text": "Promise<void>"
                }
              },
              "description": "Fetches the ${moduleNamePlural} for all agents in the DHT\\n\\nYou can subscribe to \`know${moduleNamePluralTitleCase}\` to get updated with all the ${moduleNamePlural} when this call is done\\n\\nWarning! Can be very slow"
            },
            {
              "kind": "method",
              "name": "fetchAgent${moduleNameTitleCase}",
              "return": {
                "type": {
                  "text": "Promise<${moduleNameTitleCase} | undefined>"
                }
              },
              "parameters": [
                {
                  "name": "agentPubKey",
                  "type": {
                    "text": "AgentPubKeyB64"
                  }
                }
              ],
              "description": "Fetches the ${moduleName} for the given agent"
            },
            {
              "kind": "method",
              "name": "fetchAgents${moduleNamePluralTitleCase}",
              "return": {
                "type": {
                  "text": "Promise<void>"
                }
              },
              "parameters": [
                {
                  "name": "agentPubKeys",
                  "type": {
                    "text": "AgentPubKeyB64[]"
                  }
                }
              ],
              "description": "Fetches the ${moduleNamePlural} for the given agents in the DHT\\n\\nYou can subscribe to know${moduleNamePluralTitleCase} to get updated with all the ${moduleNamePlural} when this call is done\\n\\nUse this over \`fetchAgent${moduleNameTitleCase}\` when fetching multiple ${moduleNamePlural}, as it will be more performant"
            },
            {
              "kind": "method",
              "name": "fetchMy${moduleNameTitleCase}",
              "return": {
                "type": {
                  "text": "Promise<void>"
                }
              },
              "description": "Fetch my ${moduleName}\\n\\nYou can subscribe to \`my${moduleNameTitleCase}\` to get updated with my ${moduleName}"
            },
            {
              "kind": "method",
              "name": "search${moduleNamePluralTitleCase}",
              "return": {
                "type": {
                  "text": ""
                }
              },
              "parameters": [
                {
                  "name": "nicknamePrefix",
                  "type": {
                    "text": "string"
                  },
                  "description": "must be of at least 3 characters"
                }
              ],
              "description": "Search the ${moduleNamePlural} for the agent with nicknames starting with the given nicknamePrefix"
            },
            {
              "kind": "method",
              "name": "create${moduleNameTitleCase}",
              "return": {
                "type": {
                  "text": "Promise<void>"
                }
              },
              "parameters": [
                {
                  "name": "${moduleName}",
                  "type": {
                    "text": "${moduleNameTitleCase}"
                  },
                  "description": "${moduleName} to be created"
                }
              ],
              "description": "Create my ${moduleName}\\n\\nNote that there is no guarantee on nickname uniqness"
            },
            {
              "kind": "method",
              "name": "update${moduleNameTitleCase}",
              "return": {
                "type": {
                  "text": "Promise<void>"
                }
              },
              "parameters": [
                {
                  "name": "${moduleName}",
                  "type": {
                    "text": "${moduleNameTitleCase}"
                  },
                  "description": "${moduleName} to be created"
                }
              ],
              "description": "Update my ${moduleName}"
            }
          ]
        }
      ],
      "exports": [
        {
          "kind": "js",
          "name": "${moduleNamePluralTitleCase}Store",
          "declaration": {
            "name": "${moduleNamePluralTitleCase}Store",
            "module": "src/${kebabPlural_}store.ts"
          }
        }
      ]
    },
    {
      "kind": "javascript-module",
      "path": "src/shim.d.ts",
      "declarations": [],
      "exports": []
    },
    {
      "kind": "javascript-module",
      "path": "src/types.ts",
      "declarations": [],
      "exports": []
    },
    {
      "kind": "javascript-module",
      "path": "src/definitions/agent-avatar.ts",
      "declarations": [
        {
          "kind": "class",
          "description": "",
          "name": "AA",
          "superclass": {
            "name": "AgentAvatar",
            "module": "/src/elements/agent-avatar"
          },
          "tagName": "agent-avatar",
          "customElement": true,
          "attributes": [
            {
              "name": "agent-pub-key",
              "type": {
                "text": "AgentPubKeyB64"
              },
              "description": "REQUIRED. The public key identifying the agent whose ${moduleName} is going to be shown.",
              "fieldName": "agentPubKey",
              "inheritedFrom": {
                "name": "AgentAvatar",
                "module": "src/elements/agent-avatar.ts"
              }
            },
            {
              "name": "size",
              "type": {
                "text": "number"
              },
              "default": "32",
              "description": "Size of the avatar image in pixels.",
              "fieldName": "size",
              "inheritedFrom": {
                "name": "AgentAvatar",
                "module": "src/elements/agent-avatar.ts"
              }
            },
            {
              "name": "store",
              "type": {
                "text": "${moduleNamePluralTitleCase}Store"
              },
              "description": "\`${moduleNamePluralTitleCase}Store\` that is requested via context.\\nOnly set this property if you want to override the store requested via context.",
              "fieldName": "store",
              "inheritedFrom": {
                "name": "AgentAvatar",
                "module": "src/elements/agent-avatar.ts"
              }
            }
          ],
          "members": [
            {
              "kind": "field",
              "name": "agentPubKey",
              "type": {
                "text": "AgentPubKeyB64"
              },
              "description": "REQUIRED. The public key identifying the agent whose ${moduleName} is going to be shown.",
              "attribute": "agent-pub-key",
              "inheritedFrom": {
                "name": "AgentAvatar",
                "module": "src/elements/agent-avatar.ts"
              }
            },
            {
              "kind": "field",
              "name": "size",
              "type": {
                "text": "number"
              },
              "default": "32",
              "description": "Size of the avatar image in pixels.",
              "attribute": "size",
              "inheritedFrom": {
                "name": "AgentAvatar",
                "module": "src/elements/agent-avatar.ts"
              }
            },
            {
              "kind": "field",
              "name": "store",
              "type": {
                "text": "${moduleNamePluralTitleCase}Store"
              },
              "description": "\`${moduleNamePluralTitleCase}Store\` that is requested via context.\\nOnly set this property if you want to override the store requested via context.",
              "attribute": "store",
              "inheritedFrom": {
                "name": "AgentAvatar",
                "module": "src/elements/agent-avatar.ts"
              }
            },
            {
              "kind": "field",
              "name": "${moduleNameSnakeCase}",
              "privacy": "private",
              "default": "new StoreSubscriber(this, () =>\\n    this.store?.${moduleName}Of(this.agentPubKey)\\n  )",
              "inheritedFrom": {
                "name": "AgentAvatar",
                "module": "src/elements/agent-avatar.ts"
              }
            }
          ]
        }
      ],
      "exports": [
        {
          "kind": "custom-element-definition",
          "name": "agent-avatar",
          "declaration": {
            "name": "AA",
            "module": "src/definitions/agent-avatar.ts"
          }
        }
      ]
    },
    {
      "kind": "javascript-module",
      "path": "src/definitions/create${_kebab}.ts",
      "declarations": [
        {
          "kind": "class",
          "description": "",
          "name": "CP",
          "superclass": {
            "name": "Create${moduleNameTitleCase}",
            "module": "/src/elements/create${_kebab}"
          },
          "tagName": "create${_kebab}",
          "customElement": true,
          "attributes": [
            {
              "name": "store",
              "type": {
                "text": "${moduleNamePluralTitleCase}Store"
              },
              "description": "\`${moduleNamePluralTitleCase}Store\` that is requested via context.\\nOnly set this property if you want to override the store requested via context.",
              "fieldName": "store",
              "inheritedFrom": {
                "name": "Create${moduleNameTitleCase}",
                "module": "src/elements/create${_kebab}.ts"
              }
            }
          ],
          "members": [
            {
              "kind": "field",
              "name": "store",
              "type": {
                "text": "${moduleNamePluralTitleCase}Store"
              },
              "description": "\`${moduleNamePluralTitleCase}Store\` that is requested via context.\\nOnly set this property if you want to override the store requested via context.",
              "attribute": "store",
              "inheritedFrom": {
                "name": "Create${moduleNameTitleCase}",
                "module": "src/elements/create${_kebab}.ts"
              }
            },
            {
              "kind": "method",
              "name": "create${moduleNameTitleCase}",
              "parameters": [
                {
                  "name": "${moduleName}",
                  "type": {
                    "text": "${moduleNameTitleCase}"
                  }
                }
              ],
              "description": "Private properties",
              "inheritedFrom": {
                "name": "Create${moduleNameTitleCase}",
                "module": "src/elements/create${_kebab}.ts"
              }
            }
          ],
          "events": [
            {
              "name": "${kebabSingular_}created",
              "type": {
                "text": "CustomEvent"
              },
              "description": "Fired after the ${moduleName} has been created. Detail will have this shape: { ${moduleName}: { nickname, fields } }",
              "inheritedFrom": {
                "name": "Create${moduleNameTitleCase}",
                "module": "src/elements/create${_kebab}.ts"
              }
            }
          ]
        }
      ],
      "exports": [
        {
          "kind": "custom-element-definition",
          "name": "create${_kebab}",
          "declaration": {
            "name": "CP",
            "module": "src/definitions/create${_kebab}.ts"
          }
        }
      ]
    },
    {
      "kind": "javascript-module",
      "path": "src/definitions/holo-identicon.ts",
      "declarations": [
        {
          "kind": "class",
          "description": "",
          "name": "HI",
          "superclass": {
            "name": "HoloIdenticon",
            "module": "/src/elements/holo-identicon"
          },
          "tagName": "holo-identicon",
          "customElement": true,
          "attributes": [
            {
              "name": "hash",
              "type": {
                "text": "HoloHashB64"
              },
              "description": "REQUIRED. The hash that will be converted to an identicon.",
              "fieldName": "hash",
              "inheritedFrom": {
                "name": "HoloIdenticon",
                "module": "src/elements/holo-identicon.ts"
              }
            },
            {
              "name": "size",
              "type": {
                "text": "number"
              },
              "default": "32",
              "description": "Size of the identicon in pixels.",
              "fieldName": "size",
              "inheritedFrom": {
                "name": "HoloIdenticon",
                "module": "src/elements/holo-identicon.ts"
              }
            },
            {
              "name": "shape",
              "type": {
                "text": "'square' | 'circle'"
              },
              "default": "'circle'",
              "description": "Shape of the identicon.",
              "fieldName": "shape",
              "inheritedFrom": {
                "name": "HoloIdenticon",
                "module": "src/elements/holo-identicon.ts"
              }
            }
          ],
          "members": [
            {
              "kind": "field",
              "name": "hash",
              "type": {
                "text": "HoloHashB64"
              },
              "description": "REQUIRED. The hash that will be converted to an identicon.",
              "attribute": "hash",
              "inheritedFrom": {
                "name": "HoloIdenticon",
                "module": "src/elements/holo-identicon.ts"
              }
            },
            {
              "kind": "field",
              "name": "size",
              "type": {
                "text": "number"
              },
              "default": "32",
              "description": "Size of the identicon in pixels.",
              "attribute": "size",
              "inheritedFrom": {
                "name": "HoloIdenticon",
                "module": "src/elements/holo-identicon.ts"
              }
            },
            {
              "kind": "field",
              "name": "shape",
              "type": {
                "text": "'square' | 'circle'"
              },
              "default": "'circle'",
              "description": "Shape of the identicon.",
              "attribute": "shape",
              "inheritedFrom": {
                "name": "HoloIdenticon",
                "module": "src/elements/holo-identicon.ts"
              }
            },
            {
              "kind": "field",
              "name": "_canvas",
              "type": {
                "text": "HTMLCanvasElement"
              },
              "privacy": "private",
              "inheritedFrom": {
                "name": "HoloIdenticon",
                "module": "src/elements/holo-identicon.ts"
              }
            }
          ]
        }
      ],
      "exports": [
        {
          "kind": "custom-element-definition",
          "name": "holo-identicon",
          "declaration": {
            "name": "HI",
            "module": "src/definitions/holo-identicon.ts"
          }
        }
      ]
    },
    {
      "kind": "javascript-module",
      "path": "src/definitions/list${_kebab}s.ts",
      "declarations": [
        {
          "kind": "class",
          "description": "",
          "name": "LP",
          "superclass": {
            "name": "List${moduleNamePluralTitleCase}",
            "module": "/src/elements/list${_kebab}s"
          },
          "tagName": "list${_kebab}s",
          "customElement": true,
          "attributes": [
            {
              "name": "store",
              "type": {
                "text": "${moduleNamePluralTitleCase}Store"
              },
              "description": "\`${moduleNamePluralTitleCase}Store\` that is requested via context.\\nOnly set this property if you want to override the store requested via context.",
              "fieldName": "store",
              "inheritedFrom": {
                "name": "List${moduleNamePluralTitleCase}",
                "module": "src/elements/list${_kebab}s.ts"
              }
            }
          ],
          "members": [
            {
              "kind": "field",
              "name": "store",
              "type": {
                "text": "${moduleNamePluralTitleCase}Store"
              },
              "description": "\`${moduleNamePluralTitleCase}Store\` that is requested via context.\\nOnly set this property if you want to override the store requested via context.",
              "attribute": "store",
              "inheritedFrom": {
                "name": "List${moduleNamePluralTitleCase}",
                "module": "src/elements/list${_kebab}s.ts"
              }
            },
            {
              "kind": "field",
              "name": "_loading",
              "type": {
                "text": "boolean"
              },
              "privacy": "private",
              "default": "true",
              "description": "Private properties",
              "inheritedFrom": {
                "name": "List${moduleNamePluralTitleCase}",
                "module": "src/elements/list${_kebab}s.ts"
              }
            },
            {
              "kind": "field",
              "name": "_all${moduleNamePluralTitleCase}",
              "privacy": "private",
              "default": "new StoreSubscriber(\\n    this,\\n    () => this.store?.known${moduleNamePluralTitleCase}\\n  )",
              "inheritedFrom": {
                "name": "List${moduleNamePluralTitleCase}",
                "module": "src/elements/list${_kebab}s.ts"
              }
            },
            {
              "kind": "method",
              "name": "initials",
              "return": {
                "type": {
                  "text": "string"
                }
              },
              "parameters": [
                {
                  "name": "nickname",
                  "type": {
                    "text": "string"
                  }
                }
              ],
              "inheritedFrom": {
                "name": "List${moduleNamePluralTitleCase}",
                "module": "src/elements/list${_kebab}s.ts"
              }
            },
            {
              "kind": "method",
              "name": "fireAgentSelected",
              "parameters": [
                {
                  "name": "index",
                  "type": {
                    "text": "number"
                  }
                }
              ],
              "inheritedFrom": {
                "name": "List${moduleNamePluralTitleCase}",
                "module": "src/elements/list${_kebab}s.ts"
              }
            }
          ],
          "events": [
            {
              "name": "agent-selected",
              "type": {
                "text": "CustomEvent"
              },
              "description": "Fired when the user selects an agent from the list. Detail will have this shape: { agentPubKey: 'uhCAkSEspAJks5Q8863Jg1RJhuJHJpFWzwDJkxVjVSk9JueU' }",
              "inheritedFrom": {
                "name": "List${moduleNamePluralTitleCase}",
                "module": "src/elements/list${_kebab}s.ts"
              }
            }
          ]
        }
      ],
      "exports": [
        {
          "kind": "custom-element-definition",
          "name": "list${_kebab}s",
          "declaration": {
            "name": "LP",
            "module": "src/definitions/list${_kebab}s.ts"
          }
        }
      ]
    },
    {
      "kind": "javascript-module",
      "path": "src/definitions/my${_kebab}.ts",
      "declarations": [
        {
          "kind": "class",
          "description": "",
          "name": "MP",
          "superclass": {
            "name": "My${moduleNameTitleCase}",
            "module": "/src/elements/my${_kebab}"
          },
          "tagName": "my${_kebab}",
          "customElement": true,
          "attributes": [
            {
              "name": "store",
              "type": {
                "text": "${moduleNamePluralTitleCase}Store"
              },
              "description": "\`${moduleNamePluralTitleCase}Store\` that is requested via context.\\nOnly set this property if you want to override the store requested via context.",
              "fieldName": "store",
              "inheritedFrom": {
                "name": "My${moduleNameTitleCase}",
                "module": "src/elements/my${_kebab}.ts"
              }
            }
          ],
          "members": [
            {
              "kind": "field",
              "name": "store",
              "type": {
                "text": "${moduleNamePluralTitleCase}Store"
              },
              "description": "\`${moduleNamePluralTitleCase}Store\` that is requested via context.\\nOnly set this property if you want to override the store requested via context.",
              "attribute": "store",
              "inheritedFrom": {
                "name": "My${moduleNameTitleCase}",
                "module": "src/elements/my${_kebab}.ts"
              }
            },
            {
              "kind": "field",
              "name": "_editing",
              "type": {
                "text": "boolean"
              },
              "privacy": "private",
              "default": "false",
              "description": "Private properties",
              "inheritedFrom": {
                "name": "My${moduleNameTitleCase}",
                "module": "src/elements/my${_kebab}.ts"
              }
            }
          ]
        }
      ],
      "exports": [
        {
          "kind": "custom-element-definition",
          "name": "my${_kebab}",
          "declaration": {
            "name": "MP",
            "module": "src/definitions/my${_kebab}.ts"
          }
        }
      ]
    },
    {
      "kind": "javascript-module",
      "path": "src/definitions/${kebabSingular_}detail.ts",
      "declarations": [
        {
          "kind": "class",
          "description": "",
          "name": "PD",
          "superclass": {
            "name": "${moduleNameTitleCase}Detail",
            "module": "/src/elements/${kebabSingular_}detail"
          },
          "tagName": "${kebabSingular_}detail",
          "customElement": true,
          "attributes": [
            {
              "name": "agent-pub-key",
              "type": {
                "text": "AgentPubKeyB64"
              },
              "description": "REQUIRED. Public key identifying the agent for which the ${moduleName} should be shown",
              "fieldName": "agentPubKey",
              "inheritedFrom": {
                "name": "${moduleNameTitleCase}Detail",
                "module": "src/elements/${kebabSingular_}detail.ts"
              }
            },
            {
              "name": "store",
              "type": {
                "text": "${moduleNamePluralTitleCase}Store"
              },
              "description": "\`${moduleNamePluralTitleCase}Store\` that is requested via context.\\nOnly set this property if you want to override the store requested via context.",
              "fieldName": "store",
              "inheritedFrom": {
                "name": "${moduleNameTitleCase}Detail",
                "module": "src/elements/${kebabSingular_}detail.ts"
              }
            }
          ],
          "members": [
            {
              "kind": "field",
              "name": "agentPubKey",
              "type": {
                "text": "AgentPubKeyB64"
              },
              "description": "REQUIRED. Public key identifying the agent for which the ${moduleName} should be shown",
              "attribute": "agent-pub-key",
              "inheritedFrom": {
                "name": "${moduleNameTitleCase}Detail",
                "module": "src/elements/${kebabSingular_}detail.ts"
              }
            },
            {
              "kind": "field",
              "name": "store",
              "type": {
                "text": "${moduleNamePluralTitleCase}Store"
              },
              "description": "\`${moduleNamePluralTitleCase}Store\` that is requested via context.\\nOnly set this property if you want to override the store requested via context.",
              "attribute": "store",
              "inheritedFrom": {
                "name": "${moduleNameTitleCase}Detail",
                "module": "src/elements/${kebabSingular_}detail.ts"
              }
            },
            {
              "kind": "field",
              "name": "_loading",
              "type": {
                "text": "boolean"
              },
              "privacy": "private",
              "default": "true",
              "description": "Private properties",
              "inheritedFrom": {
                "name": "${moduleNameTitleCase}Detail",
                "module": "src/elements/${kebabSingular_}detail.ts"
              }
            },
            {
              "kind": "field",
              "name": "_agent${moduleNameTitleCase}",
              "privacy": "private",
              "default": "new StoreSubscriber(this, () =>\\n    this.store?.${moduleName}Of(this.agentPubKey)\\n  )",
              "inheritedFrom": {
                "name": "${moduleNameTitleCase}Detail",
                "module": "src/elements/${kebabSingular_}detail.ts"
              }
            },
            {
              "kind": "method",
              "name": "getAdditionalFields",
              "return": {
                "type": {
                  "text": "Dictionary<string>"
                }
              },
              "inheritedFrom": {
                "name": "${moduleNameTitleCase}Detail",
                "module": "src/elements/${kebabSingular_}detail.ts"
              }
            },
            {
              "kind": "method",
              "name": "renderAdditionalField",
              "parameters": [
                {
                  "name": "fieldId",
                  "type": {
                    "text": "string"
                  }
                },
                {
                  "name": "fieldValue",
                  "type": {
                    "text": "string"
                  }
                }
              ],
              "inheritedFrom": {
                "name": "${moduleNameTitleCase}Detail",
                "module": "src/elements/${kebabSingular_}detail.ts"
              }
            }
          ]
        }
      ],
      "exports": [
        {
          "kind": "custom-element-definition",
          "name": "${kebabSingular_}detail",
          "declaration": {
            "name": "PD",
            "module": "src/definitions/${kebabSingular_}detail.ts"
          }
        }
      ]
    },
    {
      "kind": "javascript-module",
      "path": "src/definitions/${kebabSingular_}prompt.ts",
      "declarations": [
        {
          "kind": "class",
          "description": "",
          "name": "PP",
          "superclass": {
            "name": "${moduleNameTitleCase}Prompt",
            "module": "/src/elements/${kebabSingular_}prompt"
          },
          "tagName": "${kebabSingular_}prompt",
          "customElement": true,
          "attributes": [
            {
              "name": "store",
              "type": {
                "text": "${moduleNamePluralTitleCase}Store"
              },
              "description": "\`${moduleNamePluralTitleCase}Store\` that is requested via context.\\nOnly set this property if you want to override the store requested via context.",
              "fieldName": "store",
              "inheritedFrom": {
                "name": "${moduleNameTitleCase}Prompt",
                "module": "src/elements/${kebabSingular_}prompt.ts"
              }
            }
          ],
          "members": [
            {
              "kind": "field",
              "name": "store",
              "type": {
                "text": "${moduleNamePluralTitleCase}Store"
              },
              "description": "\`${moduleNamePluralTitleCase}Store\` that is requested via context.\\nOnly set this property if you want to override the store requested via context.",
              "attribute": "store",
              "inheritedFrom": {
                "name": "${moduleNameTitleCase}Prompt",
                "module": "src/elements/${kebabSingular_}prompt.ts"
              }
            },
            {
              "kind": "field",
              "name": "_loading",
              "type": {
                "text": "boolean"
              },
              "privacy": "private",
              "default": "true",
              "description": "Private properties",
              "inheritedFrom": {
                "name": "${moduleNameTitleCase}Prompt",
                "module": "src/elements/${kebabSingular_}prompt.ts"
              }
            },
            {
              "kind": "field",
              "name": "_my${moduleNameTitleCase}",
              "privacy": "private",
              "default": "new StoreSubscriber(this, () => this.store?.my${moduleNameTitleCase})",
              "inheritedFrom": {
                "name": "${moduleNameTitleCase}Prompt",
                "module": "src/elements/${kebabSingular_}prompt.ts"
              }
            },
            {
              "kind": "method",
              "name": "renderPrompt",
              "inheritedFrom": {
                "name": "${moduleNameTitleCase}Prompt",
                "module": "src/elements/${kebabSingular_}prompt.ts"
              }
            }
          ]
        }
      ],
      "exports": [
        {
          "kind": "custom-element-definition",
          "name": "${kebabSingular_}prompt",
          "declaration": {
            "name": "PP",
            "module": "src/definitions/${kebabSingular_}prompt.ts"
          }
        }
      ]
    },
    {
      "kind": "javascript-module",
      "path": "src/definitions/search-agent.ts",
      "declarations": [
        {
          "kind": "class",
          "description": "",
          "name": "SA",
          "superclass": {
            "name": "SearchAgent",
            "module": "/src/elements/search-agent"
          },
          "tagName": "search-agent",
          "customElement": true,
          "attributes": [
            {
              "name": "clear-on-select",
              "type": {
                "text": "boolean"
              },
              "default": "false",
              "description": "Whether to clear the field when an agent is selected.",
              "fieldName": "clearOnSelect",
              "attribute": "clear-on-select",
              "inheritedFrom": {
                "name": "SearchAgent",
                "module": "src/elements/search-agent.ts"
              }
            },
            {
              "name": "include-myself",
              "type": {
                "text": "boolean"
              },
              "default": "false",
              "description": "Whether to include my own agent as a possible agent to select.",
              "fieldName": "includeMyself",
              "attribute": "include-myself",
              "inheritedFrom": {
                "name": "SearchAgent",
                "module": "src/elements/search-agent.ts"
              }
            },
            {
              "name": "field-label",
              "type": {
                "text": "string"
              },
              "default": "'Search agent'",
              "description": "Label for the agent searching field.",
              "fieldName": "fieldLabel",
              "attribute": "field-label",
              "inheritedFrom": {
                "name": "SearchAgent",
                "module": "src/elements/search-agent.ts"
              }
            },
            {
              "name": "store",
              "type": {
                "text": "${moduleNamePluralTitleCase}Store"
              },
              "description": "\`${moduleNamePluralTitleCase}Store\` that is requested via context.\\nOnly set this property if you want to override the store requested via context.",
              "fieldName": "store",
              "inheritedFrom": {
                "name": "SearchAgent",
                "module": "src/elements/search-agent.ts"
              }
            }
          ],
          "members": [
            {
              "kind": "field",
              "name": "clearOnSelect",
              "type": {
                "text": "boolean"
              },
              "default": "false",
              "description": "Whether to clear the field when an agent is selected.",
              "attribute": "clear-on-select",
              "inheritedFrom": {
                "name": "SearchAgent",
                "module": "src/elements/search-agent.ts"
              }
            },
            {
              "kind": "field",
              "name": "includeMyself",
              "type": {
                "text": "boolean"
              },
              "default": "false",
              "description": "Whether to include my own agent as a possible agent to select.",
              "attribute": "include-myself",
              "inheritedFrom": {
                "name": "SearchAgent",
                "module": "src/elements/search-agent.ts"
              }
            },
            {
              "kind": "field",
              "name": "fieldLabel",
              "type": {
                "text": "string"
              },
              "default": "'Search agent'",
              "description": "Label for the agent searching field.",
              "attribute": "field-label",
              "inheritedFrom": {
                "name": "SearchAgent",
                "module": "src/elements/search-agent.ts"
              }
            },
            {
              "kind": "field",
              "name": "store",
              "type": {
                "text": "${moduleNamePluralTitleCase}Store"
              },
              "description": "\`${moduleNamePluralTitleCase}Store\` that is requested via context.\\nOnly set this property if you want to override the store requested via context.",
              "attribute": "store",
              "inheritedFrom": {
                "name": "SearchAgent",
                "module": "src/elements/search-agent.ts"
              }
            },
            {
              "kind": "field",
              "name": "_known${moduleNamePluralTitleCase}",
              "privacy": "private",
              "default": "new StoreSubscriber(\\n    this,\\n    () => this.store?.known${moduleNamePluralTitleCase}\\n  )",
              "description": "Private properties",
              "inheritedFrom": {
                "name": "SearchAgent",
                "module": "src/elements/search-agent.ts"
              }
            },
            {
              "kind": "field",
              "name": "_filteredAgents",
              "type": {
                "text": "Array<Agent${moduleNameTitleCase}>"
              },
              "privacy": "private",
              "inheritedFrom": {
                "name": "SearchAgent",
                "module": "src/elements/search-agent.ts"
              }
            },
            {
              "kind": "field",
              "name": "_currentFilter",
              "type": {
                "text": "string | undefined"
              },
              "privacy": "private",
              "default": "undefined",
              "inheritedFrom": {
                "name": "SearchAgent",
                "module": "src/elements/search-agent.ts"
              }
            },
            {
              "kind": "field",
              "name": "_lastSearchedPrefix",
              "type": {
                "text": "string | undefined"
              },
              "privacy": "private",
              "default": "undefined",
              "inheritedFrom": {
                "name": "SearchAgent",
                "module": "src/elements/search-agent.ts"
              }
            },
            {
              "kind": "field",
              "name": "_textField",
              "type": {
                "text": "TextField"
              },
              "privacy": "private",
              "inheritedFrom": {
                "name": "SearchAgent",
                "module": "src/elements/search-agent.ts"
              }
            },
            {
              "kind": "field",
              "name": "_overlay",
              "type": {
                "text": "MenuSurface"
              },
              "privacy": "private",
              "inheritedFrom": {
                "name": "SearchAgent",
                "module": "src/elements/search-agent.ts"
              }
            },
            {
              "kind": "method",
              "name": "searchAgents",
              "return": {
                "type": {
                  "text": "Promise<void>"
                }
              },
              "parameters": [
                {
                  "name": "nicknamePrefix",
                  "type": {
                    "text": "string"
                  }
                }
              ],
              "inheritedFrom": {
                "name": "SearchAgent",
                "module": "src/elements/search-agent.ts"
              }
            },
            {
              "kind": "method",
              "name": "onFilterChange",
              "inheritedFrom": {
                "name": "SearchAgent",
                "module": "src/elements/search-agent.ts"
              }
            },
            {
              "kind": "method",
              "name": "onUsernameSelected",
              "parameters": [
                {
                  "name": "agent",
                  "type": {
                    "text": "Agent${moduleNameTitleCase}"
                  }
                }
              ],
              "inheritedFrom": {
                "name": "SearchAgent",
                "module": "src/elements/search-agent.ts"
              }
            }
          ],
          "events": [
            {
              "name": "agent-selected",
              "type": {
                "text": "CustomEvent"
              },
              "description": "Fired when the user selects some agent. Detail will have this shape: { agentPubKey: 'uhCAkSEspAJks5Q8863Jg1RJhuJHJpFWzwDJkxVjVSk9JueU' }",
              "inheritedFrom": {
                "name": "SearchAgent",
                "module": "src/elements/search-agent.ts"
              }
            }
          ]
        }
      ],
      "exports": [
        {
          "kind": "custom-element-definition",
          "name": "search-agent",
          "declaration": {
            "name": "SA",
            "module": "src/definitions/search-agent.ts"
          }
        }
      ]
    },
    {
      "kind": "javascript-module",
      "path": "src/definitions/update${_kebab}.ts",
      "declarations": [
        {
          "kind": "class",
          "description": "",
          "name": "UP",
          "superclass": {
            "name": "Update${moduleNameTitleCase}",
            "module": "/src/elements/update${_kebab}"
          },
          "tagName": "update${_kebab}",
          "customElement": true,
          "attributes": [
            {
              "name": "store",
              "type": {
                "text": "${moduleNamePluralTitleCase}Store"
              },
              "description": "\`${moduleNamePluralTitleCase}Store\` that is requested via context.\\nOnly set this property if you want to override the store requested via context.",
              "fieldName": "store",
              "inheritedFrom": {
                "name": "Update${moduleNameTitleCase}",
                "module": "src/elements/update${_kebab}.ts"
              }
            }
          ],
          "members": [
            {
              "kind": "field",
              "name": "store",
              "type": {
                "text": "${moduleNamePluralTitleCase}Store"
              },
              "description": "\`${moduleNamePluralTitleCase}Store\` that is requested via context.\\nOnly set this property if you want to override the store requested via context.",
              "attribute": "store",
              "inheritedFrom": {
                "name": "Update${moduleNameTitleCase}",
                "module": "src/elements/update${_kebab}.ts"
              }
            },
            {
              "kind": "field",
              "name": "_loading",
              "type": {
                "text": "boolean"
              },
              "privacy": "private",
              "default": "true",
              "description": "Private properties",
              "inheritedFrom": {
                "name": "Update${moduleNameTitleCase}",
                "module": "src/elements/update${_kebab}.ts"
              }
            },
            {
              "kind": "field",
              "name": "_my${moduleNameTitleCase}",
              "privacy": "private",
              "default": "new StoreSubscriber(this, () => this.store?.my${moduleNameTitleCase})",
              "inheritedFrom": {
                "name": "Update${moduleNameTitleCase}",
                "module": "src/elements/update${_kebab}.ts"
              }
            },
            {
              "kind": "method",
              "name": "update${moduleNameTitleCase}",
              "parameters": [
                {
                  "name": "${moduleName}",
                  "type": {
                    "text": "${moduleNameTitleCase}"
                  }
                }
              ],
              "inheritedFrom": {
                "name": "Update${moduleNameTitleCase}",
                "module": "src/elements/update${_kebab}.ts"
              }
            }
          ],
          "events": [
            {
              "name": "${kebabSingular_}updated",
              "type": {
                "text": "CustomEvent"
              },
              "description": "Fired after the ${moduleName} has been created. Detail will have this shape: { ${moduleName}: { nickname, fields } }",
              "inheritedFrom": {
                "name": "Update${moduleNameTitleCase}",
                "module": "src/elements/update${_kebab}.ts"
              }
            }
          ]
        }
      ],
      "exports": [
        {
          "kind": "custom-element-definition",
          "name": "update${_kebab}",
          "declaration": {
            "name": "UP",
            "module": "src/definitions/update${_kebab}.ts"
          }
        }
      ]
    },
    {
      "kind": "javascript-module",
      "path": "src/elements/agent-avatar.ts",
      "declarations": [
        {
          "kind": "class",
          "description": "",
          "name": "AgentAvatar",
          "members": [
            {
              "kind": "field",
              "name": "agentPubKey",
              "type": {
                "text": "AgentPubKeyB64"
              },
              "description": "REQUIRED. The public key identifying the agent whose ${moduleName} is going to be shown.",
              "attribute": "agent-pub-key"
            },
            {
              "kind": "field",
              "name": "size",
              "type": {
                "text": "number"
              },
              "default": "32",
              "description": "Size of the avatar image in pixels.",
              "attribute": "size"
            },
            {
              "kind": "field",
              "name": "store",
              "type": {
                "text": "${moduleNamePluralTitleCase}Store"
              },
              "description": "\`${moduleNamePluralTitleCase}Store\` that is requested via context.\\nOnly set this property if you want to override the store requested via context.",
              "attribute": "store"
            },
            {
              "kind": "field",
              "name": "${moduleNameSnakeCase}",
              "privacy": "private",
              "default": "new StoreSubscriber(this, () =>\\n    this.store?.${moduleName}Of(this.agentPubKey)\\n  )"
            }
          ],
          "attributes": [
            {
              "name": "agent-pub-key",
              "type": {
                "text": "AgentPubKeyB64"
              },
              "description": "REQUIRED. The public key identifying the agent whose ${moduleName} is going to be shown.",
              "fieldName": "agentPubKey"
            },
            {
              "name": "size",
              "type": {
                "text": "number"
              },
              "default": "32",
              "description": "Size of the avatar image in pixels.",
              "fieldName": "size"
            },
            {
              "name": "store",
              "type": {
                "text": "${moduleNamePluralTitleCase}Store"
              },
              "description": "\`${moduleNamePluralTitleCase}Store\` that is requested via context.\\nOnly set this property if you want to override the store requested via context.",
              "fieldName": "store"
            }
          ],
          "mixins": [
            {
              "name": "ScopedElementsMixin",
              "package": "@open-wc/scoped-elements"
            }
          ],
          "superclass": {
            "name": "LitElement",
            "package": "lit"
          },
          "customElement": true
        }
      ],
      "exports": [
        {
          "kind": "js",
          "name": "AgentAvatar",
          "declaration": {
            "name": "AgentAvatar",
            "module": "src/elements/agent-avatar.ts"
          }
        }
      ]
    },
    {
      "kind": "javascript-module",
      "path": "src/elements/create${_kebab}.ts",
      "declarations": [
        {
          "kind": "class",
          "description": "A custom element that fires event on value change.",
          "name": "Create${moduleNameTitleCase}",
          "members": [
            {
              "kind": "field",
              "name": "store",
              "type": {
                "text": "${moduleNamePluralTitleCase}Store"
              },
              "description": "\`${moduleNamePluralTitleCase}Store\` that is requested via context.\\nOnly set this property if you want to override the store requested via context.",
              "attribute": "store"
            },
            {
              "kind": "method",
              "name": "create${moduleNameTitleCase}",
              "parameters": [
                {
                  "name": "${moduleName}",
                  "type": {
                    "text": "${moduleNameTitleCase}"
                  }
                }
              ],
              "description": "Private properties"
            }
          ],
          "events": [
            {
              "name": "${kebabSingular_}created",
              "type": {
                "text": "CustomEvent"
              },
              "description": "Fired after the ${moduleName} has been created. Detail will have this shape: { ${moduleName}: { nickname, fields } }"
            }
          ],
          "attributes": [
            {
              "name": "store",
              "type": {
                "text": "${moduleNamePluralTitleCase}Store"
              },
              "description": "\`${moduleNamePluralTitleCase}Store\` that is requested via context.\\nOnly set this property if you want to override the store requested via context.",
              "fieldName": "store"
            }
          ],
          "mixins": [
            {
              "name": "ScopedElementsMixin",
              "package": "@open-wc/scoped-elements"
            }
          ],
          "superclass": {
            "name": "LitElement",
            "package": "lit"
          },
          "tagName": "create${_kebab}",
          "customElement": true
        }
      ],
      "exports": [
        {
          "kind": "js",
          "name": "Create${moduleNameTitleCase}",
          "declaration": {
            "name": "Create${moduleNameTitleCase}",
            "module": "src/elements/create${_kebab}.ts"
          }
        }
      ]
    },
    {
      "kind": "javascript-module",
      "path": "src/elements/edit${_kebab}.ts",
      "declarations": [
        {
          "kind": "class",
          "description": "",
          "name": "Edit${moduleNameTitleCase}",
          "members": [
            {
              "kind": "field",
              "name": "${moduleName}",
              "type": {
                "text": "${moduleNameTitleCase} | undefined"
              },
              "description": "The ${moduleName} to be edited.",
              "attribute": "${moduleName}"
            },
            {
              "kind": "field",
              "name": "save${moduleNameTitleCase}Label",
              "type": {
                "text": "string"
              },
              "default": "'Save ${moduleNameTitleCase}'",
              "description": "Label for the save ${moduleName} button.",
              "attribute": "save${_kebab}-label"
            },
            {
              "kind": "field",
              "name": "store",
              "type": {
                "text": "${moduleNamePluralTitleCase}Store"
              },
              "description": "\`${moduleNamePluralTitleCase}Store\` that is requested via context.\\nOnly set this property if you want to override the store requested via context.",
              "attribute": "store"
            },
            {
              "kind": "field",
              "name": "_nicknameField",
              "type": {
                "text": "TextField"
              },
              "privacy": "private",
              "description": "Private properties"
            },
            {
              "kind": "field",
              "name": "_existingUsernames",
              "type": {
                "text": "{ [key: string]: boolean }"
              },
              "privacy": "private",
              "default": "{}"
            },
            {
              "kind": "field",
              "name": "_avatarFilePicker",
              "type": {
                "text": "HTMLInputElement"
              },
              "privacy": "private"
            },
            {
              "kind": "field",
              "name": "_avatar",
              "type": {
                "text": "string | undefined"
              },
              "privacy": "private"
            },
            {
              "kind": "method",
              "name": "onAvatarUploaded"
            },
            {
              "kind": "method",
              "name": "avatarMode"
            },
            {
              "kind": "method",
              "name": "renderAvatar"
            },
            {
              "kind": "method",
              "name": "shouldSaveButtonBeEnabled"
            },
            {
              "kind": "method",
              "name": "textfieldToFieldId",
              "return": {
                "type": {
                  "text": "string"
                }
              },
              "parameters": [
                {
                  "name": "field",
                  "type": {
                    "text": "TextField"
                  }
                }
              ]
            },
            {
              "kind": "method",
              "name": "getAdditionalFieldsValues",
              "return": {
                "type": {
                  "text": "Dictionary<string>"
                }
              }
            },
            {
              "kind": "method",
              "name": "getAdditionalTextFields",
              "return": {
                "type": {
                  "text": "Dictionary<TextField>"
                }
              }
            },
            {
              "kind": "method",
              "name": "fireSave${moduleNameTitleCase}"
            },
            {
              "kind": "method",
              "name": "renderField",
              "parameters": [
                {
                  "name": "fieldName",
                  "type": {
                    "text": "string"
                  }
                }
              ]
            }
          ],
          "events": [
            {
              "name": "save${_kebab}",
              "type": {
                "text": "CustomEvent"
              },
              "description": "Fired when the save ${moduleName} button is clicked"
            }
          ],
          "attributes": [
            {
              "name": "${moduleName}",
              "type": {
                "text": "${moduleNameTitleCase} | undefined"
              },
              "description": "The ${moduleName} to be edited.",
              "fieldName": "${moduleName}"
            },
            {
              "name": "save${_kebab}-label",
              "type": {
                "text": "string"
              },
              "default": "'Save ${moduleNameTitleCase}'",
              "description": "Label for the save ${moduleName} button.",
              "fieldName": "save${moduleNameTitleCase}Label"
            },
            {
              "name": "store",
              "type": {
                "text": "${moduleNamePluralTitleCase}Store"
              },
              "description": "\`${moduleNamePluralTitleCase}Store\` that is requested via context.\\nOnly set this property if you want to override the store requested via context.",
              "fieldName": "store"
            }
          ],
          "mixins": [
            {
              "name": "ScopedElementsMixin",
              "package": "@open-wc/scoped-elements"
            }
          ],
          "superclass": {
            "name": "LitElement",
            "package": "lit"
          },
          "tagName": "edit${_kebab}",
          "customElement": true
        }
      ],
      "exports": [
        {
          "kind": "js",
          "name": "Edit${moduleNameTitleCase}",
          "declaration": {
            "name": "Edit${moduleNameTitleCase}",
            "module": "src/elements/edit${_kebab}.ts"
          }
        }
      ]
    },
    {
      "kind": "javascript-module",
      "path": "src/elements/holo-identicon.ts",
      "declarations": [
        {
          "kind": "class",
          "description": "",
          "name": "HoloIdenticon",
          "members": [
            {
              "kind": "field",
              "name": "hash",
              "type": {
                "text": "HoloHashB64"
              },
              "description": "REQUIRED. The hash that will be converted to an identicon.",
              "attribute": "hash"
            },
            {
              "kind": "field",
              "name": "size",
              "type": {
                "text": "number"
              },
              "default": "32",
              "description": "Size of the identicon in pixels.",
              "attribute": "size"
            },
            {
              "kind": "field",
              "name": "shape",
              "type": {
                "text": "'square' | 'circle'"
              },
              "default": "'circle'",
              "description": "Shape of the identicon.",
              "attribute": "shape"
            },
            {
              "kind": "field",
              "name": "_canvas",
              "type": {
                "text": "HTMLCanvasElement"
              },
              "privacy": "private"
            }
          ],
          "attributes": [
            {
              "name": "hash",
              "type": {
                "text": "HoloHashB64"
              },
              "description": "REQUIRED. The hash that will be converted to an identicon.",
              "fieldName": "hash"
            },
            {
              "name": "size",
              "type": {
                "text": "number"
              },
              "default": "32",
              "description": "Size of the identicon in pixels.",
              "fieldName": "size"
            },
            {
              "name": "shape",
              "type": {
                "text": "'square' | 'circle'"
              },
              "default": "'circle'",
              "description": "Shape of the identicon.",
              "fieldName": "shape"
            }
          ],
          "superclass": {
            "name": "LitElement",
            "package": "lit"
          },
          "customElement": true
        }
      ],
      "exports": [
        {
          "kind": "js",
          "name": "HoloIdenticon",
          "declaration": {
            "name": "HoloIdenticon",
            "module": "src/elements/holo-identicon.ts"
          }
        }
      ]
    },
    {
      "kind": "javascript-module",
      "path": "src/elements/list${_kebab}s.ts",
      "declarations": [
        {
          "kind": "class",
          "description": "",
          "name": "List${moduleNamePluralTitleCase}",
          "members": [
            {
              "kind": "field",
              "name": "store",
              "type": {
                "text": "${moduleNamePluralTitleCase}Store"
              },
              "description": "\`${moduleNamePluralTitleCase}Store\` that is requested via context.\\nOnly set this property if you want to override the store requested via context.",
              "attribute": "store"
            },
            {
              "kind": "field",
              "name": "_loading",
              "type": {
                "text": "boolean"
              },
              "privacy": "private",
              "default": "true",
              "description": "Private properties"
            },
            {
              "kind": "field",
              "name": "_all${moduleNamePluralTitleCase}",
              "privacy": "private",
              "default": "new StoreSubscriber(\\n    this,\\n    () => this.store?.known${moduleNamePluralTitleCase}\\n  )"
            },
            {
              "kind": "method",
              "name": "initials",
              "return": {
                "type": {
                  "text": "string"
                }
              },
              "parameters": [
                {
                  "name": "nickname",
                  "type": {
                    "text": "string"
                  }
                }
              ]
            },
            {
              "kind": "method",
              "name": "fireAgentSelected",
              "parameters": [
                {
                  "name": "index",
                  "type": {
                    "text": "number"
                  }
                }
              ]
            }
          ],
          "events": [
            {
              "name": "agent-selected",
              "type": {
                "text": "CustomEvent"
              },
              "description": "Fired when the user selects an agent from the list. Detail will have this shape: { agentPubKey: 'uhCAkSEspAJks5Q8863Jg1RJhuJHJpFWzwDJkxVjVSk9JueU' }"
            }
          ],
          "attributes": [
            {
              "name": "store",
              "type": {
                "text": "${moduleNamePluralTitleCase}Store"
              },
              "description": "\`${moduleNamePluralTitleCase}Store\` that is requested via context.\\nOnly set this property if you want to override the store requested via context.",
              "fieldName": "store"
            }
          ],
          "mixins": [
            {
              "name": "ScopedElementsMixin",
              "package": "@open-wc/scoped-elements"
            }
          ],
          "superclass": {
            "name": "LitElement",
            "package": "lit"
          },
          "tagName": "list${_kebab}s",
          "customElement": true
        }
      ],
      "exports": [
        {
          "kind": "js",
          "name": "List${moduleNamePluralTitleCase}",
          "declaration": {
            "name": "List${moduleNamePluralTitleCase}",
            "module": "src/elements/list${_kebab}s.ts"
          }
        }
      ]
    },
    {
      "kind": "javascript-module",
      "path": "src/elements/my${_kebab}.ts",
      "declarations": [
        {
          "kind": "class",
          "description": "",
          "name": "My${moduleNameTitleCase}",
          "members": [
            {
              "kind": "field",
              "name": "store",
              "type": {
                "text": "${moduleNamePluralTitleCase}Store"
              },
              "description": "\`${moduleNamePluralTitleCase}Store\` that is requested via context.\\nOnly set this property if you want to override the store requested via context.",
              "attribute": "store"
            },
            {
              "kind": "field",
              "name": "_editing",
              "type": {
                "text": "boolean"
              },
              "privacy": "private",
              "default": "false",
              "description": "Private properties"
            }
          ],
          "attributes": [
            {
              "name": "store",
              "type": {
                "text": "${moduleNamePluralTitleCase}Store"
              },
              "description": "\`${moduleNamePluralTitleCase}Store\` that is requested via context.\\nOnly set this property if you want to override the store requested via context.",
              "fieldName": "store"
            }
          ],
          "mixins": [
            {
              "name": "ScopedElementsMixin",
              "package": "@open-wc/scoped-elements"
            }
          ],
          "superclass": {
            "name": "LitElement",
            "package": "lit"
          },
          "tagName": "${kebabSingular_}detail",
          "customElement": true
        }
      ],
      "exports": [
        {
          "kind": "js",
          "name": "My${moduleNameTitleCase}",
          "declaration": {
            "name": "My${moduleNameTitleCase}",
            "module": "src/elements/my${_kebab}.ts"
          }
        }
      ]
    },
    {
      "kind": "javascript-module",
      "path": "src/elements/${kebabSingular_}detail.ts",
      "declarations": [
        {
          "kind": "class",
          "description": "",
          "name": "${moduleNameTitleCase}Detail",
          "members": [
            {
              "kind": "field",
              "name": "agentPubKey",
              "type": {
                "text": "AgentPubKeyB64"
              },
              "description": "REQUIRED. Public key identifying the agent for which the ${moduleName} should be shown",
              "attribute": "agent-pub-key"
            },
            {
              "kind": "field",
              "name": "store",
              "type": {
                "text": "${moduleNamePluralTitleCase}Store"
              },
              "description": "\`${moduleNamePluralTitleCase}Store\` that is requested via context.\\nOnly set this property if you want to override the store requested via context.",
              "attribute": "store"
            },
            {
              "kind": "field",
              "name": "_loading",
              "type": {
                "text": "boolean"
              },
              "privacy": "private",
              "default": "true",
              "description": "Private properties"
            },
            {
              "kind": "field",
              "name": "_agent${moduleNameTitleCase}",
              "privacy": "private",
              "default": "new StoreSubscriber(this, () =>\\n    this.store?.${moduleName}Of(this.agentPubKey)\\n  )"
            },
            {
              "kind": "method",
              "name": "getAdditionalFields",
              "return": {
                "type": {
                  "text": "Dictionary<string>"
                }
              }
            },
            {
              "kind": "method",
              "name": "renderAdditionalField",
              "parameters": [
                {
                  "name": "fieldId",
                  "type": {
                    "text": "string"
                  }
                },
                {
                  "name": "fieldValue",
                  "type": {
                    "text": "string"
                  }
                }
              ]
            }
          ],
          "attributes": [
            {
              "name": "agent-pub-key",
              "type": {
                "text": "AgentPubKeyB64"
              },
              "description": "REQUIRED. Public key identifying the agent for which the ${moduleName} should be shown",
              "fieldName": "agentPubKey"
            },
            {
              "name": "store",
              "type": {
                "text": "${moduleNamePluralTitleCase}Store"
              },
              "description": "\`${moduleNamePluralTitleCase}Store\` that is requested via context.\\nOnly set this property if you want to override the store requested via context.",
              "fieldName": "store"
            }
          ],
          "mixins": [
            {
              "name": "ScopedElementsMixin",
              "package": "@open-wc/scoped-elements"
            }
          ],
          "superclass": {
            "name": "LitElement",
            "package": "lit"
          },
          "tagName": "${kebabSingular_}detail",
          "customElement": true
        }
      ],
      "exports": [
        {
          "kind": "js",
          "name": "${moduleNameTitleCase}Detail",
          "declaration": {
            "name": "${moduleNameTitleCase}Detail",
            "module": "src/elements/${kebabSingular_}detail.ts"
          }
        }
      ]
    },
    {
      "kind": "javascript-module",
      "path": "src/elements/${kebabSingular_}prompt.ts",
      "declarations": [
        {
          "kind": "class",
          "description": "",
          "name": "${moduleNameTitleCase}Prompt",
          "slots": [
            {
              "description": "Will be displayed above the create${_kebab} form when the user is prompted with it",
              "name": "hero"
            }
          ],
          "members": [
            {
              "kind": "field",
              "name": "store",
              "type": {
                "text": "${moduleNamePluralTitleCase}Store"
              },
              "description": "\`${moduleNamePluralTitleCase}Store\` that is requested via context.\\nOnly set this property if you want to override the store requested via context.",
              "attribute": "store"
            },
            {
              "kind": "field",
              "name": "_loading",
              "type": {
                "text": "boolean"
              },
              "privacy": "private",
              "default": "true",
              "description": "Private properties"
            },
            {
              "kind": "field",
              "name": "_my${moduleNameTitleCase}",
              "privacy": "private",
              "default": "new StoreSubscriber(this, () => this.store?.my${moduleNameTitleCase})"
            },
            {
              "kind": "method",
              "name": "renderPrompt"
            }
          ],
          "attributes": [
            {
              "name": "store",
              "type": {
                "text": "${moduleNamePluralTitleCase}Store"
              },
              "description": "\`${moduleNamePluralTitleCase}Store\` that is requested via context.\\nOnly set this property if you want to override the store requested via context.",
              "fieldName": "store"
            }
          ],
          "mixins": [
            {
              "name": "ScopedElementsMixin",
              "package": "@open-wc/scoped-elements"
            }
          ],
          "superclass": {
            "name": "LitElement",
            "package": "lit"
          },
          "tagName": "${kebabSingular_}prompt",
          "customElement": true
        }
      ],
      "exports": [
        {
          "kind": "js",
          "name": "${moduleNameTitleCase}Prompt",
          "declaration": {
            "name": "${moduleNameTitleCase}Prompt",
            "module": "src/elements/${kebabSingular_}prompt.ts"
          }
        }
      ]
    },
    {
      "kind": "javascript-module",
      "path": "src/elements/search-agent.ts",
      "declarations": [
        {
          "kind": "class",
          "description": "",
          "name": "SearchAgent",
          "members": [
            {
              "kind": "field",
              "name": "clearOnSelect",
              "type": {
                "text": "boolean"
              },
              "default": "false",
              "description": "Whether to clear the field when an agent is selected.",
              "attribute": "clear-on-select"
            },
            {
              "kind": "field",
              "name": "includeMyself",
              "type": {
                "text": "boolean"
              },
              "default": "false",
              "description": "Whether to include my own agent as a possible agent to select.",
              "attribute": "include-myself"
            },
            {
              "kind": "field",
              "name": "fieldLabel",
              "type": {
                "text": "string"
              },
              "default": "'Search agent'",
              "description": "Label for the agent searching field.",
              "attribute": "field-label"
            },
            {
              "kind": "field",
              "name": "store",
              "type": {
                "text": "${moduleNamePluralTitleCase}Store"
              },
              "description": "\`${moduleNamePluralTitleCase}Store\` that is requested via context.\\nOnly set this property if you want to override the store requested via context.",
              "attribute": "store"
            },
            {
              "kind": "field",
              "name": "_known${moduleNamePluralTitleCase}",
              "privacy": "private",
              "default": "new StoreSubscriber(\\n    this,\\n    () => this.store?.known${moduleNamePluralTitleCase}\\n  )",
              "description": "Private properties"
            },
            {
              "kind": "field",
              "name": "_filteredAgents",
              "type": {
                "text": "Array<Agent${moduleNameTitleCase}>"
              },
              "privacy": "private"
            },
            {
              "kind": "field",
              "name": "_currentFilter",
              "type": {
                "text": "string | undefined"
              },
              "privacy": "private",
              "default": "undefined"
            },
            {
              "kind": "field",
              "name": "_lastSearchedPrefix",
              "type": {
                "text": "string | undefined"
              },
              "privacy": "private",
              "default": "undefined"
            },
            {
              "kind": "field",
              "name": "_textField",
              "type": {
                "text": "TextField"
              },
              "privacy": "private"
            },
            {
              "kind": "field",
              "name": "_overlay",
              "type": {
                "text": "MenuSurface"
              },
              "privacy": "private"
            },
            {
              "kind": "method",
              "name": "searchAgents",
              "return": {
                "type": {
                  "text": "Promise<void>"
                }
              },
              "parameters": [
                {
                  "name": "nicknamePrefix",
                  "type": {
                    "text": "string"
                  }
                }
              ]
            },
            {
              "kind": "method",
              "name": "onFilterChange"
            },
            {
              "kind": "method",
              "name": "onUsernameSelected",
              "parameters": [
                {
                  "name": "agent",
                  "type": {
                    "text": "Agent${moduleNameTitleCase}"
                  }
                }
              ]
            }
          ],
          "events": [
            {
              "name": "agent-selected",
              "type": {
                "text": "CustomEvent"
              },
              "description": "Fired when the user selects some agent. Detail will have this shape: { agentPubKey: 'uhCAkSEspAJks5Q8863Jg1RJhuJHJpFWzwDJkxVjVSk9JueU' }"
            }
          ],
          "attributes": [
            {
              "name": "clear-on-select",
              "type": {
                "text": "boolean"
              },
              "default": "false",
              "description": "Whether to clear the field when an agent is selected.",
              "fieldName": "clearOnSelect",
              "attribute": "clear-on-select"
            },
            {
              "name": "include-myself",
              "type": {
                "text": "boolean"
              },
              "default": "false",
              "description": "Whether to include my own agent as a possible agent to select.",
              "fieldName": "includeMyself",
              "attribute": "include-myself"
            },
            {
              "name": "field-label",
              "type": {
                "text": "string"
              },
              "default": "'Search agent'",
              "description": "Label for the agent searching field.",
              "fieldName": "fieldLabel",
              "attribute": "field-label"
            },
            {
              "name": "store",
              "type": {
                "text": "${moduleNamePluralTitleCase}Store"
              },
              "description": "\`${moduleNamePluralTitleCase}Store\` that is requested via context.\\nOnly set this property if you want to override the store requested via context.",
              "fieldName": "store"
            }
          ],
          "mixins": [
            {
              "name": "ScopedElementsMixin",
              "package": "@open-wc/scoped-elements"
            }
          ],
          "superclass": {
            "name": "LitElement",
            "package": "lit"
          },
          "tagName": "search-agent",
          "customElement": true
        }
      ],
      "exports": [
        {
          "kind": "js",
          "name": "SearchAgent",
          "declaration": {
            "name": "SearchAgent",
            "module": "src/elements/search-agent.ts"
          }
        }
      ]
    },
    {
      "kind": "javascript-module",
      "path": "src/elements/update${_kebab}.ts",
      "declarations": [
        {
          "kind": "class",
          "description": "",
          "name": "Update${moduleNameTitleCase}",
          "members": [
            {
              "kind": "field",
              "name": "store",
              "type": {
                "text": "${moduleNamePluralTitleCase}Store"
              },
              "description": "\`${moduleNamePluralTitleCase}Store\` that is requested via context.\\nOnly set this property if you want to override the store requested via context.",
              "attribute": "store"
            },
            {
              "kind": "field",
              "name": "_loading",
              "type": {
                "text": "boolean"
              },
              "privacy": "private",
              "default": "true",
              "description": "Private properties"
            },
            {
              "kind": "field",
              "name": "_my${moduleNameTitleCase}",
              "privacy": "private",
              "default": "new StoreSubscriber(this, () => this.store?.my${moduleNameTitleCase})"
            },
            {
              "kind": "method",
              "name": "update${moduleNameTitleCase}",
              "parameters": [
                {
                  "name": "${moduleName}",
                  "type": {
                    "text": "${moduleNameTitleCase}"
                  }
                }
              ]
            }
          ],
          "events": [
            {
              "name": "${kebabSingular_}updated",
              "type": {
                "text": "CustomEvent"
              },
              "description": "Fired after the ${moduleName} has been created. Detail will have this shape: { ${moduleName}: { nickname, fields } }"
            }
          ],
          "attributes": [
            {
              "name": "store",
              "type": {
                "text": "${moduleNamePluralTitleCase}Store"
              },
              "description": "\`${moduleNamePluralTitleCase}Store\` that is requested via context.\\nOnly set this property if you want to override the store requested via context.",
              "fieldName": "store"
            }
          ],
          "mixins": [
            {
              "name": "ScopedElementsMixin",
              "package": "@open-wc/scoped-elements"
            }
          ],
          "superclass": {
            "name": "LitElement",
            "package": "lit"
          },
          "tagName": "update${_kebab}",
          "customElement": true
        }
      ],
      "exports": [
        {
          "kind": "js",
          "name": "Update${moduleNameTitleCase}",
          "declaration": {
            "name": "Update${moduleNameTitleCase}",
            "module": "src/elements/update${_kebab}.ts"
          }
        }
      ]
    },
    {
      "kind": "javascript-module",
      "path": "test/mocks/index.js",
      "declarations": [
        {
          "kind": "function",
          "name": "getAppWebsocket"
        }
      ],
      "exports": [
        {
          "kind": "js",
          "name": "getAppWebsocket",
          "declaration": {
            "name": "getAppWebsocket",
            "module": "test/mocks/index.js"
          }
        }
      ]
    },
    {
      "kind": "javascript-module",
      "path": "test/mocks/${moduleNamePlural}.mock.js",
      "declarations": [
        {
          "kind": "class",
          "description": "",
          "name": "${moduleNamePluralTitleCase}Mock",
          "members": [
            {
              "kind": "method",
              "name": "create${moduleNameSnakeCase}",
              "parameters": [
                {
                  "name": "{ username }"
                },
                {
                  "name": "provenance"
                }
              ]
            },
            {
              "kind": "method",
              "name": "search${moduleNameSnakeCase}s",
              "parameters": [
                {
                  "name": "{ username_prefix }"
                }
              ]
            },
            {
              "kind": "method",
              "name": "get_my${moduleNameSnakeCase}",
              "parameters": [
                {
                  "name": "_"
                },
                {
                  "name": "provenance"
                }
              ]
            },
            {
              "kind": "method",
              "name": "get_agent${moduleNameSnakeCase}",
              "parameters": [
                {
                  "name": "{ agent_address }"
                }
              ]
            },
            {
              "kind": "method",
              "name": "findAgent",
              "parameters": [
                {
                  "name": "agent_address"
                }
              ]
            },
            {
              "kind": "field",
              "name": "agents",
              "type": {
                "text": "array"
              },
              "default": "[]"
            }
          ]
        }
      ],
      "exports": [
        {
          "kind": "js",
          "name": "${moduleNamePluralTitleCase}Mock",
          "declaration": {
            "name": "${moduleNamePluralTitleCase}Mock",
            "module": "test/mocks/${moduleNamePlural}.mock.js"
          }
        }
      ]
    },
    {
      "kind": "javascript-module",
      "path": "src/elements/utils/image.ts",
      "declarations": [
        {
          "kind": "function",
          "name": "resizeAndExport",
          "parameters": [
            {
              "name": "img",
              "type": {
                "text": "HTMLImageElement"
              }
            }
          ]
        }
      ],
      "exports": [
        {
          "kind": "js",
          "name": "resizeAndExport",
          "declaration": {
            "name": "resizeAndExport",
            "module": "src/elements/utils/image.ts"
          }
        }
      ]
    },
    {
      "kind": "javascript-module",
      "path": "src/elements/utils/shared-styles.ts",
      "declarations": [
        {
          "kind": "variable",
          "name": "sharedStyles",
          "default": "css\`\\n  .row {\\n    display: flex;\\n    flex-direction: row;\\n  }\\n  .column {\\n    display: flex;\\n    flex-direction: column;\\n  }\\n  .small-margin {\\n    margin-top: 6px;\\n  }\\n  .big-margin {\\n    margin-top: 23px;\\n  }\\n\\n  .fill {\\n    flex: 1;\\n    height: 100%;\\n  }\\n\\n  .title {\\n    font-size: 20px;\\n  }\\n\\n  .center-content {\\n    align-items: center;\\n    justify-content: center;\\n  }\\n\\n  .placeholder {\\n    color: rgba(0, 0, 0, 0.7);\\n  }\\n\\n  .label {\\n    color: var(--mdc-text-field-label-ink-color, rgba(0, 0, 0, 0.6));\\n    font-family: var(\\n      --mdc-typography-caption-font-family,\\n      var(--mdc-typography-font-family, Roboto, sans-serif)\\n    );\\n    font-size: var(--mdc-typography-caption-font-size, 0.79rem);\\n    font-weight: var(--mdc-typography-caption-font-weight, 400);\\n  }\\n\`"
        }
      ],
      "exports": [
        {
          "kind": "js",
          "name": "sharedStyles",
          "declaration": {
            "name": "sharedStyles",
            "module": "src/elements/utils/shared-styles.ts"
          }
        }
      ]
    }
  ]
}
`
});
    