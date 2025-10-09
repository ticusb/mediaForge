export default function Header({
    user,
    isPro,
    freeUploadsRemaining,
    onUpgradeClick,
}) {
    return (
        <header className="bg-white border-b border-gray-200 px-8 py-4">
            <div className="flex justify-between items-center">
                {/* Left side - Welcome message */}
                <div>
                    <h2 className="text-xl font-semibold text-gray-800">
                        {user
                            ? `Welcome back, ${user.email.split("@")[0]}`
                            : "Welcome to MediaForge"}
                    </h2>
                    <p className="text-sm text-gray-500 mt-0.5">
                        Professional media editing, right in your browser
                    </p>
                </div>

                {/* Right side - Status and actions */}
                <div className="flex items-center gap-4">
                    {/* Free tier indicator */}
                    {!isPro && (
                        <div className="flex items-center gap-2 px-4 py-2 bg-gray-100 rounded-full">
                            <span className="text-sm font-medium text-gray-700">
                                {freeUploadsRemaining === Infinity
                                    ? "Free Tier"
                                    : `${freeUploadsRemaining} upload${
                                          freeUploadsRemaining === 1 ? "" : "s"
                                      } left`}
                            </span>
                        </div>
                    )}

                    {/* User status badge */}
                    {user && (
                        <div
                            className={`px-4 py-2 rounded-full text-sm font-semibold ${
                                isPro
                                    ? "bg-green-500 text-white"
                                    : "bg-gray-100 text-gray-700"
                            }`}
                        >
                            {isPro ? "⭐ Pro Member" : "Free Account"}
                        </div>
                    )}

                    {/* Upgrade button */}
                    {!isPro && (
                        <button
                            onClick={onUpgradeClick}
                            className="bg-blue-500 text-white px-6 py-2.5 rounded-lg text-sm font-semibold hover:bg-blue-600 transition-colors shadow-sm hover:shadow-md"
                        >
                            Upgrade to Pro
                        </button>
                    )}

                    {/* Export button (placeholder) */}
                    <button className="bg-green-500 text-white px-6 py-2.5 rounded-lg text-sm font-semibold hover:bg-green-600 transition-colors shadow-sm hover:shadow-md">
                        Export
                    </button>
                </div>
            </div>
        </header>
    );
}
