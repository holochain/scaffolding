import { CargoImport } from "@source-craft/cargo";

export type VocabularyRustGenerators = Record<string, RustTypeGenerator>;

export interface RustTypeGenerator {
  imports: CargoImport[];
  defineType: string;
  referenceType: string;
}
