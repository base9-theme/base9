(async function main() {
  const mustache = require('mustache');
  const yaml = require('js-yaml');
  const _ = require('lodash');
  const fs = require('fs');
  const Ajv = require('ajv/dist/2019');
  const { compile } = require('json-schema-to-typescript');
  const ajv = new Ajv();
  const schemaYml = yaml.load(readFile("scripts/schema.yml"));
  const validate = ajv.compile(schemaYml);
  await compile(schemaYml, 'Config')
    .then(ts => fs.writeFileSync('scripts/config.ts', ts))


  function readFile(path) {
      return fs.readFileSync(path, {encoding:'utf8', flag:'r'});
  }
  const TEMPLATES = "templates";
  const template = readFile("scripts/README.md.mustache");

  const root_data = {
      tags: [
          {name: "Terminals", key: "terminal", list: [] },
          {name: "Code Editors", key: "editor", list: [] },
          {name: "Graphical UI", key: "gui", list: [] },
          {name: "Libraries", key: "library", list: [] },
      ]
  }

  for(let app of fs.readdirSync("templates")) {
      const config = yaml.load(readFile(TEMPLATES + "/" + app + "/config.yml"));
      if(!validate(config)) {
          console.log(app)
          console.log(validate.errors)
          continue;
      }
      config.app.key = app;

      // update root_data
      let tag_line = root_data.tags.find(x => x.key === config.type);
      tag_line.list.push(config);

      const readme = mustache.render(template, config);
      if(readme.length > 1) {
          fs.writeFileSync(TEMPLATES + "/" + app + "/README.md", readme);
      }
  }

  _.forEach(root_data.tags, tag_line => {
      _.sortBy(tag_line.list, a => a.app.name );
  });

  fs.writeFileSync("README.md", mustache.render(readFile("scripts/ROOT_README.md.mustache"), root_data));
})();
