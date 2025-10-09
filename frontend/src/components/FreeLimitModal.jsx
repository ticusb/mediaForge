import Modal from "./Modal";

export default function FreeLimitModal({
    isOpen,
    onClose,
    onUpgrade,
    remaining,
}) {
    const isLimitReached = remaining === 0;

    return (
        <Modal
            isOpen={isOpen}
            onClose={isLimitReached ? undefined : onClose}
            size="md"
        >
            <div className="text-center mb-8">
                <div className="text-6xl mb-4">
                    {isLimitReached ? "🔒" : "⚠️"}
                </div>
                <h2 className="text-3xl font-bold text-gray-800 mb-2">
                    {isLimitReached ? "Free Limit Reached" : "Almost There!"}
                </h2>
                <p className="text-lg text-gray-600">
                    {isLimitReached
                        ? "You've used all your free uploads for today"
                        : `You have ${remaining} free upload${
                              remaining === 1 ? "" : "s"
                          } remaining today`}
                </p>
            </div>

            {/* Pro benefits card */}
            <div className="bg-gradient-to-br from-blue-50 to-green-50 border-2 border-blue-200 rounded-2xl p-6 mb-6">
                <div className="flex items-center gap-2 mb-4">
                    <span className="text-2xl">⭐</span>
                    <h3 className="text-xl font-bold text-gray-800">
                        Upgrade to Pro
                    </h3>
                </div>

                <div className="text-3xl font-bold text-blue-600 mb-4">
                    $9.99
                    <span className="text-lg text-gray-600 font-normal">
                        /month
                    </span>
                </div>

                <div className="space-y-3">
                    <div className="flex items-start gap-3">
                        <div className="w-6 h-6 bg-green-500 rounded-full flex items-center justify-center flex-shrink-0 mt-0.5">
                            <svg
                                className="w-4 h-4 text-white"
                                fill="none"
                                strokeLinecap="round"
                                strokeLinejoin="round"
                                strokeWidth="2"
                                viewBox="0 0 24 24"
                                stroke="currentColor"
                            >
                                <path d="M5 13l4 4L19 7"></path>
                            </svg>
                        </div>
                        <div>
                            <div className="font-semibold text-gray-800">
                                Unlimited Uploads
                            </div>
                            <div className="text-sm text-gray-600">
                                Process as many files as you need
                            </div>
                        </div>
                    </div>

                    <div className="flex items-start gap-3">
                        <div className="w-6 h-6 bg-green-500 rounded-full flex items-center justify-center flex-shrink-0 mt-0.5">
                            <svg
                                className="w-4 h-4 text-white"
                                fill="none"
                                strokeLinecap="round"
                                strokeLinejoin="round"
                                strokeWidth="2"
                                viewBox="0 0 24 24"
                                stroke="currentColor"
                            >
                                <path d="M5 13l4 4L19 7"></path>
                            </svg>
                        </div>
                        <div>
                            <div className="font-semibold text-gray-800">
                                No Watermarks
                            </div>
                            <div className="text-sm text-gray-600">
                                Professional, clean exports
                            </div>
                        </div>
                    </div>

                    <div className="flex items-start gap-3">
                        <div className="w-6 h-6 bg-green-500 rounded-full flex items-center justify-center flex-shrink-0 mt-0.5">
                            <svg
                                className="w-4 h-4 text-white"
                                fill="none"
                                strokeLinecap="round"
                                strokeLinejoin="round"
                                strokeWidth="2"
                                viewBox="0 0 24 24"
                                stroke="currentColor"
                            >
                                <path d="M5 13l4 4L19 7"></path>
                            </svg>
                        </div>
                        <div>
                            <div className="font-semibold text-gray-800">
                                Priority Processing
                            </div>
                            <div className="text-sm text-gray-600">
                                Faster turnaround times
                            </div>
                        </div>
                    </div>

                    <div className="flex items-start gap-3">
                        <div className="w-6 h-6 bg-green-500 rounded-full flex items-center justify-center flex-shrink-0 mt-0.5">
                            <svg
                                className="w-4 h-4 text-white"
                                fill="none"
                                strokeLinecap="round"
                                strokeLinejoin="round"
                                strokeWidth="2"
                                viewBox="0 0 24 24"
                                stroke="currentColor"
                            >
                                <path d="M5 13l4 4L19 7"></path>
                            </svg>
                        </div>
                        <div>
                            <div className="font-semibold text-gray-800">
                                Advanced Tools
                            </div>
                            <div className="text-sm text-gray-600">
                                Access to all editing features
                            </div>
                        </div>
                    </div>

                    <div className="flex items-start gap-3">
                        <div className="w-6 h-6 bg-green-500 rounded-full flex items-center justify-center flex-shrink-0 mt-0.5">
                            <svg
                                className="w-4 h-4 text-white"
                                fill="none"
                                strokeLinecap="round"
                                strokeLinejoin="round"
                                strokeWidth="2"
                                viewBox="0 0 24 24"
                                stroke="currentColor"
                            >
                                <path d="M5 13l4 4L19 7"></path>
                            </svg>
                        </div>
                        <div>
                            <div className="font-semibold text-gray-800">
                                Save to Account
                            </div>
                            <div className="text-sm text-gray-600">
                                Store and access your work anywhere
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            {/* Action buttons */}
            <div className="space-y-3">
                <button
                    onClick={onUpgrade}
                    className="w-full bg-gradient-to-r from-blue-500 to-green-500 text-white py-4 rounded-xl font-bold text-lg hover:from-blue-600 hover:to-green-600 transition-all transform hover:scale-[1.02] shadow-lg"
                >
                    Upgrade to Pro Now
                </button>

                {!isLimitReached && (
                    <button
                        onClick={onClose}
                        className="w-full bg-white text-gray-700 py-3 rounded-xl font-semibold border-2 border-gray-300 hover:bg-gray-50 transition-colors"
                    >
                        Continue with Free ({remaining} upload
                        {remaining === 1 ? "" : "s"} left)
                    </button>
                )}
            </div>

            {/* Money back guarantee */}
            <div className="mt-6 text-center">
                <p className="text-sm text-gray-500">
                    <span className="inline-block mr-1">🛡️</span>
                    30-day money-back guarantee • Cancel anytime
                </p>
            </div>
        </Modal>
    );
}
