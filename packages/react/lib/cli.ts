

import { Command } from 'commander';

const program = new Command();

program
    .name('@supra-dev/react')
    .version('1.0.0-alpha.1')
    .description('A library to support rendering React components in non-js server environments')

program.parse(process.argv);

