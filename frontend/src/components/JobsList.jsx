export default function JobsList({ jobs }) {
    const formatDate = (dateString) => {
        const date = new Date(dateString);
        const now = new Date();
        const diffInSeconds = Math.floor((now - date) / 1000);

        if (diffInSeconds < 60) return "Just now";
        if (diffInSeconds < 3600)
            return `${Math.floor(diffInSeconds / 60)}m ago`;
        if (diffInSeconds < 86400)
            return `${Math.floor(diffInSeconds / 3600)}h ago`;
        return date.toLocaleDateString();
    };

    const getStatusBadge = (status) => {
        const statusConfig = {
            uploaded: {
                bg: "bg-green-100",
                text: "text-green-700",
                label: "Uploaded",
            },
            processing: {
                bg: "bg-blue-100",
                text: "text-blue-700",
                label: "Processing",
            },
            completed: {
                bg: "bg-green-100",
                text: "text-green-700",
                label: "Completed",
            },
            failed: { bg: "bg-red-100", text: "text-red-700", label: "Failed" },
        };

        const config = statusConfig[status] || statusConfig.uploaded;

        return (
            <span
                className={`px-3 py-1 rounded-full text-xs font-medium ${config.bg} ${config.text}`}
            >
                {config.label}
            </span>
        );
    };

    if (jobs.length === 0) {
        return (
            <div className="bg-white rounded-xl border border-gray-200 p-12 text-center">
                <div className="text-6xl mb-4">📋</div>
                <h3 className="text-xl font-semibold text-gray-800 mb-2">
                    No jobs yet
                </h3>
                <p className="text-gray-500">
                    Your processed files will appear here
                </p>
            </div>
        );
    }

    return (
        <div className="bg-white rounded-xl border border-gray-200 overflow-hidden shadow-sm">
            <div className="px-6 py-4 border-b border-gray-200 bg-gray-50">
                <h3 className="text-lg font-semibold text-gray-800">
                    Recent Jobs
                </h3>
            </div>

            <div className="divide-y divide-gray-200">
                {jobs.map((job) => (
                    <div
                        key={job.id}
                        className="px-6 py-4 hover:bg-gray-50 transition-colors"
                    >
                        <div className="flex items-center justify-between">
                            <div className="flex-1 min-w-0 mr-4">
                                <div className="flex items-center gap-3 mb-2">
                                    <span className="text-2xl">📄</span>
                                    <div className="flex-1 min-w-0">
                                        <h4 className="font-medium text-gray-800 truncate">
                                            {job.filename}
                                        </h4>
                                        <div className="flex items-center gap-3 mt-1">
                                            {getStatusBadge(job.status)}
                                            {job.watermarked && (
                                                <span className="text-xs text-gray-500">
                                                    • Watermarked (Free tier)
                                                </span>
                                            )}
                                            <span className="text-xs text-gray-400">
                                                {formatDate(job.createdAt)}
                                            </span>
                                        </div>
                                    </div>
                                </div>
                            </div>

                            <div className="flex items-center gap-2">
                                <button className="px-4 py-2 bg-blue-500 text-white rounded-lg text-sm font-medium hover:bg-blue-600 transition-colors">
                                    Download
                                </button>
                                <button className="px-3 py-2 bg-gray-100 text-gray-600 rounded-lg text-sm hover:bg-gray-200 transition-colors">
                                    ⋯
                                </button>
                            </div>
                        </div>
                    </div>
                ))}
            </div>
        </div>
    );
}
