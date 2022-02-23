import ts from 'typescript';

export function parseTsCode(code: string): ts.NodeArray<ts.Statement> {
  const sourceFile = ts.createSourceFile('sample.ts', code, ts.ScriptTarget.ES2015, /*setParentNodes */ false);
  return sourceFile.statements;
}

export function importDeclaration(importStr: string): ts.ImportDeclaration {
  return parseTsCode(importStr)[0] as ts.ImportDeclaration;
}

export function printTypescript(statements: ts.NodeArray<ts.Statement>): string {
  const resultFile = ts.createSourceFile(
    'someFileName.ts',
    '',
    ts.ScriptTarget.Latest,
    /*setParentNodes*/ false,
    ts.ScriptKind.TS,
  );
  const printer = ts.createPrinter({ newLine: ts.NewLineKind.LineFeed });

  return printer.printList(ts.ListFormat.MultiLine, statements, resultFile);
}

export function functionDeclaration(functionStr: string): ts.FunctionDeclaration {
  return parseTsCode(functionStr)[0] as ts.FunctionDeclaration;
}
