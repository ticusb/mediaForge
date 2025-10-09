import UploadZone from "./UploadZone";
import Preview from "./Preview";
import ToolPanel from "./ToolPanel";
import JobsList from "./JobsList";

export default function Workspace({
    file,
    jobs,
    selectedTool,
    isFreeLimitReached,
    onFileSelect,
    onUpload,
}) {
    return (
        <main className="flex-1 overflow-auto">
            <div className="p-8">
                {/* Main editing area */}
                <div className="grid grid-cols-1 lg:grid-cols-3 gap-6 mb-8">
                    {/* Left column - Upload/Preview */}
                    <div className="lg:col-span-2 space-y-6">
                        {!file ? (
                            <UploadZone
                                onFileSelect={onFileSelect}
                                disabled={isFreeLimitReached}
                            />
                        ) : (
                            <>
                                <Preview file={file} />

                                <div className="flex gap-4">
                                    <button
                                        onClick={onUpload}
                                        disabled={isFreeLimitReached}
                                        className={`flex-1 py-4 rounded-lg font-semibold text-lg transition-all ${
                                            isFreeLimitReached
                                                ? "bg-gray-300 text-gray-500 cursor-not-allowed"
                                                : "bg-green-500 text-white hover:bg-green-600 shadow-md hover:shadow-lg active:scale-[0.98]"
                                        }`}
                                    >
                                        {isFreeLimitReached
                                            ? "Upgrade to Process"
                                            : "Process File"}
                                    </button>

                                    <button
                                        onClick={() => onFileSelect(null)}
                                        className="px-6 py-4 bg-gray-200 text-gray-700 rounded-lg font-semibold hover:bg-gray-300 transition-colors"
                                    >
                                        Cancel
                                    </button>
                                </div>
                            </>
                        )}
                    </div>

                    {/* Right column - Tool Panel */}
                    <div>
                        <ToolPanel
                            selectedTool={selectedTool}
                            hasFile={!!file}
                        />
                    </div>
                </div>

                {/* Jobs list */}
                <JobsList jobs={jobs} />
            </div>
        </main>
    );
}
