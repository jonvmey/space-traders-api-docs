alias g := generate

version := "2.2.0"

generate:
  npx @openapitools/openapi-generator-cli generate \
      -i reference/SpaceTraders.json \
      -g rust \
      --library reqwest \
      --additional-properties=packageName=space-traders \
      --additional-properties=packageVersion={{version}} \
      --additional-properties=preferUnsignedInt=true \
      --additional-properties=bestFitInt=true \
      --additional-properties=avoidBoxedModels=true
