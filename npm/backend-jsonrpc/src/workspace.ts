// Generated file, do not edit by hand, see `xtask/codegen`
import type { Transport } from "./transport";
export interface SupportsFeatureParams {
	feature: FeatureName;
	path: RomePath;
}
export type FeatureName = "Format" | "Lint";
export interface RomePath {
	id: FileId;
	path: string;
}
/**
 * An id that points into a file database.
 */
export type FileId = number;
export interface SupportsFeatureResult {
	reason?: UnsupportedReason;
}
export type UnsupportedReason =
	| "Ignored"
	| "FeatureNotEnabled"
	| "FileNotSupported";
export interface UpdateSettingsParams {
	configuration: Configuration;
}
/**
 * The configuration that is contained inside the file `rome.json`
 */
export interface Configuration {
	/**
	 * The configuration of the formatter
	 */
	formatter?: FormatterConfiguration;
	/**
	 * Specific configuration for the JavaScript language
	 */
	javascript?: JavascriptConfiguration;
	/**
	 * The configuration for the linter
	 */
	linter?: LinterConfiguration;
}
export interface FormatterConfiguration {
	enabled?: boolean;
	/**
	 * Stores whether formatting should be allowed to proceed if a given file has syntax errors
	 */
	formatWithErrors?: boolean;
	/**
	 * A list of Unix shell style patterns. The formatter will ignore files/folders that will match these patterns.
	 */
	ignore?: string[];
	/**
	 * The size of the indentation, 2 by default
	 */
	indentSize?: number;
	/**
	 * The indent style.
	 */
	indentStyle?: PlainIndentStyle;
	/**
	 * What's the max width of a line. Defaults to 80.
	 */
	lineWidth?: LineWidth;
}
export interface JavascriptConfiguration {
	formatter?: JavascriptFormatter;
	/**
	* A list of global bindings that should be ignored by the analyzers

If defined here, they should not emit diagnostics. 
	 */
	globals?: string[];
}
export interface LinterConfiguration {
	/**
	 * if `false`, it disables the feature and the linter won't be executed. `true` by default
	 */
	enabled?: boolean;
	/**
	 * A list of Unix shell style patterns. The formatter will ignore files/folders that will match these patterns.
	 */
	ignore?: string[];
	/**
	 * List of rules
	 */
	rules?: Rules;
}
export type PlainIndentStyle = "tab" | "space";
/**
	* Validated value for the `line_width` formatter options

The allowed range of values is 1..=320 
	 */
export type LineWidth = number;
export interface JavascriptFormatter {
	/**
	 * When properties in objects are quoted. Defaults to asNeeded.
	 */
	quoteProperties?: QuoteProperties;
	/**
	 * The style for quotes. Defaults to double.
	 */
	quoteStyle?: QuoteStyle;
}
export interface Rules {
	correctness?: Correctness;
	nursery?: Nursery;
	/**
	 * It enables the lint rules recommended by Rome. `true` by default.
	 */
	recommended?: boolean;
	style?: Style;
}
export type QuoteProperties = "asNeeded" | "preserve";
export type QuoteStyle = "double" | "single";
/**
 * A list of rules that belong to this group
 */
export interface Correctness {
	noArguments?: RuleConfiguration;
	noAsyncPromiseExecutor?: RuleConfiguration;
	noCatchAssign?: RuleConfiguration;
	noCommentText?: RuleConfiguration;
	noCompareNegZero?: RuleConfiguration;
	noDebugger?: RuleConfiguration;
	noDelete?: RuleConfiguration;
	noDoubleEquals?: RuleConfiguration;
	noDupeArgs?: RuleConfiguration;
	noEmptyPattern?: RuleConfiguration;
	noExtraBooleanCast?: RuleConfiguration;
	noFunctionAssign?: RuleConfiguration;
	noImplicitBoolean?: RuleConfiguration;
	noImportAssign?: RuleConfiguration;
	noLabelVar?: RuleConfiguration;
	noMultipleSpacesInRegularExpressionLiterals?: RuleConfiguration;
	noShadowRestrictedNames?: RuleConfiguration;
	noSparseArray?: RuleConfiguration;
	noUnnecessaryContinue?: RuleConfiguration;
	noUnsafeNegation?: RuleConfiguration;
	noUnusedTemplateLiteral?: RuleConfiguration;
	/**
	 * It enables the recommended rules for this group
	 */
	recommended?: boolean;
	useBlockStatements?: RuleConfiguration;
	useSimplifiedLogicExpression?: RuleConfiguration;
	useSingleCaseStatement?: RuleConfiguration;
	useSingleVarDeclarator?: RuleConfiguration;
	useTemplate?: RuleConfiguration;
	useValidTypeof?: RuleConfiguration;
	useWhile?: RuleConfiguration;
}
/**
 * A list of rules that belong to this group
 */
export interface Nursery {
	noDangerouslySetInnerHtml?: RuleConfiguration;
	noNewSymbol?: RuleConfiguration;
	noRenderReturnValue?: RuleConfiguration;
	noUnreachable?: RuleConfiguration;
	noUnusedVariables?: RuleConfiguration;
	noUselessFragments?: RuleConfiguration;
	noVoidElementsWithChildren?: RuleConfiguration;
	/**
	 * It enables the recommended rules for this group
	 */
	recommended?: boolean;
	useButtonType?: RuleConfiguration;
	useCamelCase?: RuleConfiguration;
	useFragmentSyntax?: RuleConfiguration;
	useOptionalChain?: RuleConfiguration;
}
/**
 * A list of rules that belong to this group
 */
export interface Style {
	noNegationElse?: RuleConfiguration;
	noShoutyConstants?: RuleConfiguration;
	/**
	 * It enables the recommended rules for this group
	 */
	recommended?: boolean;
	useSelfClosingElements?: RuleConfiguration;
	useShorthandArrayType?: RuleConfiguration;
}
export type RuleConfiguration = RulePlainConfiguration | RuleWithOptions;
export type RulePlainConfiguration = "warn" | "error" | "off";
export interface RuleWithOptions {
	level: RulePlainConfiguration;
	options: any;
}
export interface OpenFileParams {
	content: string;
	language_hint?: Language;
	path: RomePath;
	version: number;
}
/**
 * Supported languages by Rome
 */
export type Language =
	| "JavaScript"
	| "JavaScriptReact"
	| "TypeScript"
	| "TypeScriptReact"
	| "Json"
	| "Unknown";
export interface ChangeFileParams {
	content: string;
	path: RomePath;
	version: number;
}
export interface CloseFileParams {
	path: RomePath;
}
export interface GetSyntaxTreeParams {
	path: RomePath;
}
export interface GetSyntaxTreeResult {
	ast: string;
	cst: string;
}
export interface GetControlFlowGraphParams {
	cursor: TextSize;
	path: RomePath;
}
export type TextSize = number;
export interface GetFormatterIRParams {
	path: RomePath;
}
export interface PullDiagnosticsParams {
	categories: RuleCategories;
	path: RomePath;
}
export type RuleCategories = RuleCategory[];
export type RuleCategory = "Syntax" | "Lint" | "Action";
export interface PullDiagnosticsResult {
	diagnostics: Diagnostic[];
}
/**
 * A diagnostic message that can give information like errors or warnings.
 */
export interface Diagnostic {
	children: SubDiagnostic[];
	code?: Category;
	file_id: FileId;
	footers: Footer[];
	primary?: SubDiagnostic;
	severity: Severity;
	suggestions: CodeSuggestion[];
	summary?: string;
	tag?: DiagnosticTag;
	title: MarkupBuf;
}
/**
 * Everything that can be added to a diagnostic, like a suggestion that will be displayed under the actual error.
 */
export interface SubDiagnostic {
	msg: MarkupBuf;
	severity: Severity;
	span: FileSpan;
}
export type Category =
	| "lint/correctness/noArguments"
	| "lint/correctness/noAsyncPromiseExecutor"
	| "lint/correctness/noCatchAssign"
	| "lint/correctness/noCommentText"
	| "lint/correctness/noCompareNegZero"
	| "lint/correctness/noDebugger"
	| "lint/correctness/noDelete"
	| "lint/correctness/noDoubleEquals"
	| "lint/correctness/noDupeArgs"
	| "lint/correctness/noEmptyPattern"
	| "lint/correctness/noExtraBooleanCast"
	| "lint/correctness/noFunctionAssign"
	| "lint/correctness/noImplicitBoolean"
	| "lint/correctness/noImportAssign"
	| "lint/correctness/noLabelVar"
	| "lint/correctness/noMultipleSpacesInRegularExpressionLiterals"
	| "lint/correctness/noShadowRestrictedNames"
	| "lint/correctness/noSparseArray"
	| "lint/correctness/noUnnecessaryContinue"
	| "lint/correctness/noUnsafeNegation"
	| "lint/correctness/noUnusedTemplateLiteral"
	| "lint/correctness/useBlockStatements"
	| "lint/correctness/useSimplifiedLogicExpression"
	| "lint/correctness/useSingleCaseStatement"
	| "lint/correctness/useSingleVarDeclarator"
	| "lint/correctness/useTemplate"
	| "lint/correctness/useValidTypeof"
	| "lint/correctness/useWhile"
	| "lint/nursery/noDangerouslySetInnerHtml"
	| "lint/nursery/noNewSymbol"
	| "lint/nursery/noRenderReturnValue"
	| "lint/nursery/noUnreachable"
	| "lint/nursery/noUnusedVariables"
	| "lint/nursery/noUselessFragments"
	| "lint/nursery/useButtonType"
	| "lint/nursery/useCamelCase"
	| "lint/nursery/useOptionalChain"
	| "lint/nursery/noVoidElementsWithChildren"
	| "lint/nursery/noChildrenProp"
	| "lint/nursery/useFragmentSyntax"
	| "lint/style/noNegationElse"
	| "lint/style/noShoutyConstants"
	| "lint/style/useSelfClosingElements"
	| "lint/style/useShorthandArrayType"
	| "files/missingHandler"
	| "format"
	| "internalError/io"
	| "internalError/fs"
	| "internalError/panic"
	| "lint"
	| "parse"
	| "suppressions/unknownGroup"
	| "suppressions/unknownRule"
	| "args/fileNotFound"
	| "flags/invalid"
	| "semanticTests";
/**
 * A note or help that is displayed under the diagnostic.
 */
export interface Footer {
	msg: MarkupBuf;
	severity: Severity;
}
/**
	* A severity level for diagnostic messages.

These are ordered in the following way: 
	 */
export type Severity = "Help" | "Note" | "Warning" | "Error" | "Bug";
/**
 * A Suggestion that is provided by rslint, and can be reported to the user, and can be automatically applied if it has the right [`Applicability`].
 */
export interface CodeSuggestion {
	applicability: Applicability;
	labels: TextRange[];
	msg: MarkupBuf;
	span: FileSpan;
	style: SuggestionStyle;
	/**
	 * If the `FileId` is `None`, it's in the same file as his parent.
	 */
	substitution: SuggestionChange;
}
export type DiagnosticTag = "Unnecessary" | "Deprecated" | "Both";
export type MarkupBuf = MarkupNodeBuf[];
/**
 * A range that is indexed in a specific file.
 */
export interface FileSpan {
	file: FileId;
	range: TextRange;
}
/**
 * Indicates how a tool should manage this suggestion.
 */
export type Applicability = "Always" | "MaybeIncorrect";
export type TextRange = [TextSize, TextSize];
export type SuggestionStyle = "Inline" | "Full";
export type SuggestionChange = { Indels: Indel[] } | { String: string };
export interface MarkupNodeBuf {
	content: string;
	elements: MarkupElement[];
}
/**
	* `InsertDelete` -- a single "atomic" change to text

Must not overlap with other `InDel`s 
	 */
export interface Indel {
	/**
	 * Refers to offsets in the original text
	 */
	delete: TextRange;
	insert: string;
}
/**
 * Enumeration of all the supported markup elements
 */
export type MarkupElement =
	| "Emphasis"
	| "Dim"
	| "Italic"
	| "Underline"
	| "Error"
	| "Success"
	| "Warn"
	| "Info"
	| "Inverse"
	| { Hyperlink: { href: string } };
export interface PullActionsParams {
	path: RomePath;
	range: TextRange;
}
export interface PullActionsResult {
	actions: CodeAction[];
}
export interface CodeAction {
	category: ActionCategory;
	rule_name: string;
	suggestion: CodeSuggestion;
}
export type ActionCategory = "QuickFix" | "Refactor";
export interface FormatFileParams {
	path: RomePath;
}
export interface Printed {
	code: string;
	range?: TextRange;
	sourcemap: SourceMarker[];
	verbatim_ranges: TextRange[];
}
/**
 * Lightweight sourcemap marker between source and output tokens
 */
export interface SourceMarker {
	/**
	 * Position of the marker in the output code
	 */
	dest: TextSize;
	/**
	 * Position of the marker in the original source
	 */
	source: TextSize;
}
export interface FormatRangeParams {
	path: RomePath;
	range: TextRange;
}
export interface FormatOnTypeParams {
	offset: TextSize;
	path: RomePath;
}
export interface FixFileParams {
	fix_file_mode: FixFileMode;
	path: RomePath;
}
/**
 * Which fixes should be applied during the analyzing phase
 */
export type FixFileMode = "SafeFixes" | "SafeAndSuggestedFixes";
export interface FixFileResult {
	/**
	 * List of all the code actions applied to the file
	 */
	actions: FixAction[];
	/**
	 * New source code for the file with all fixes applied
	 */
	code: string;
	/**
	 * number of skipped suggested fixes
	 */
	skipped_suggested_fixes: number;
}
export interface FixAction {
	/**
	 * Source range at which this action was applied
	 */
	range: TextRange;
	/**
	 * Name of the rule that emitted this code action
	 */
	rule_name: string;
}
export interface RenameParams {
	new_name: string;
	path: RomePath;
	symbol_at: TextSize;
}
export interface RenameResult {
	/**
	 * List of text edit operations to apply on the source code
	 */
	indels: Indel[];
	/**
	 * Range of source code modified by this rename operation
	 */
	range: TextRange;
}
export interface Workspace {
	supportsFeature(
		params: SupportsFeatureParams,
	): Promise<SupportsFeatureResult>;
	updateSettings(params: UpdateSettingsParams): Promise<void>;
	openFile(params: OpenFileParams): Promise<void>;
	changeFile(params: ChangeFileParams): Promise<void>;
	closeFile(params: CloseFileParams): Promise<void>;
	getSyntaxTree(params: GetSyntaxTreeParams): Promise<GetSyntaxTreeResult>;
	getControlFlowGraph(params: GetControlFlowGraphParams): Promise<string>;
	getFormatterIr(params: GetFormatterIRParams): Promise<string>;
	pullDiagnostics(
		params: PullDiagnosticsParams,
	): Promise<PullDiagnosticsResult>;
	pullActions(params: PullActionsParams): Promise<PullActionsResult>;
	formatFile(params: FormatFileParams): Promise<Printed>;
	formatRange(params: FormatRangeParams): Promise<Printed>;
	formatOnType(params: FormatOnTypeParams): Promise<Printed>;
	fixFile(params: FixFileParams): Promise<FixFileResult>;
	rename(params: RenameParams): Promise<RenameResult>;
	destroy(): void;
}
export function createWorkspace(transport: Transport): Workspace {
	return {
		supportsFeature(params) {
			return transport.request("rome/supports_feature", params);
		},
		updateSettings(params) {
			return transport.request("rome/update_settings", params);
		},
		openFile(params) {
			return transport.request("rome/open_file", params);
		},
		changeFile(params) {
			return transport.request("rome/change_file", params);
		},
		closeFile(params) {
			return transport.request("rome/close_file", params);
		},
		getSyntaxTree(params) {
			return transport.request("rome/get_syntax_tree", params);
		},
		getControlFlowGraph(params) {
			return transport.request("rome/get_control_flow_graph", params);
		},
		getFormatterIr(params) {
			return transport.request("rome/get_formatter_ir", params);
		},
		pullDiagnostics(params) {
			return transport.request("rome/pull_diagnostics", params);
		},
		pullActions(params) {
			return transport.request("rome/pull_actions", params);
		},
		formatFile(params) {
			return transport.request("rome/format_file", params);
		},
		formatRange(params) {
			return transport.request("rome/format_range", params);
		},
		formatOnType(params) {
			return transport.request("rome/format_on_type", params);
		},
		fixFile(params) {
			return transport.request("rome/fix_file", params);
		},
		rename(params) {
			return transport.request("rome/rename", params);
		},
		destroy() {
			transport.destroy();
		},
	};
}
