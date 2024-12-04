import { Command } from 'commander';
import { loadComponent, pipeComponentToStdout } from './server';

const program = new Command();

program
    .name('@supra-dev/react')
    .version('1.0.0-alpha.1')
    .description('A library to support rendering React components in non-js server environments')

program
    .command('render')
    .description('Render a React component')
    .argument('<component>', 'The path to the component to render')
    .option('-P, --props <props>', 'The props to pass to the component', '{}')
    .option('-N, --name <name>', 'The name of the component', 'default')
    .action(async (component, options) => {
        const Component = await loadComponent(component, options.name);
        const props = JSON.parse(options.props);
        await pipeComponentToStdout(<Component {...props} />);
    })


program.parse(process.argv);

