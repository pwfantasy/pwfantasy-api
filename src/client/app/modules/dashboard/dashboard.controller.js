(function() {
    "use strict";

    angular
        .module('dashboard')
        .controller('DashboardCtrl', DashboardCtrl);

    DashboardCtrl.$inject = ['user'];

    function DashboardCtrl (user) {
        var vm = this;

        vm.user = user;
    }
})();