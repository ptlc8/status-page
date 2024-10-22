const Api = (function () {
    return {
        status: {
            async getAll() {
                var res = await fetch('/api/projects');
                return await res.json();
            }
        }
    };
})();