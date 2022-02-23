import { CargoImport } from "@source-craft/cargo";

export type RustTypeGenerators = Record<string, RustTypeGenerator>;

export interface RustTypeGenerator {
  imports: CargoImport[];
  defineType: string;
  referenceType: string;
}
