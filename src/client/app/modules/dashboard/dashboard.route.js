(function() {
    "use strict";

    angular
        .module('dashboard')
        .config(configuration);

    function configuration($stateProvider) {
        $stateProvider
            .state('dashboard', {
                url: '/',
                templateUrl: '/app/modules/dashboard/dashboard.view.html',
                controller: 'DashboardCtrl',
                controllerAs: 'dashboardVm',
                resolve: {
                    user: function() {
                        return {};
                    }
                }
            });
    }
})();