(function() {
    "use strict";

    angular
        .module('header')
        .directive('header', Header);

    function Header() {
        return {
            restrict: 'E',
            controller: 'HeaderCtrl',
            controllerAs: 'headerVm',
            templateUrl: '/app/components/header/header.view.html'
        };
    }

})();
