(function() {
    "use strict";

    angular
        .module('superstars.search')
        .controller('SearchCtrl', SearchCtrl);

    SearchCtrl.$inject = ['term', 'results'];

    function SearchCtrl(term, results) {
        var vm = this;

        vm.term = term;
        vm.results = results;
    }
})();