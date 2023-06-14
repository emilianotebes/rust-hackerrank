function diagonalDiff(parametro){
    var diagonal1 = 0;
    var diagonal2 = 0;

    for(var i=0;i<parametro.length;i++){
        for(var j=0; j<parametro.length;j++){
            if(i===j){
                diagonal1 = diagonal1 + parametro[i][j];
            }
            if(i+j===parametro.length-1){
                diagonal2 = diagonal2 +  parametro[i][j];
            }
        }
    }
    console.log(diagonal2);
    console.log(diagonal1);
    return Math.abs(diagonal1-diagonal2);

}


var algo = [
    [11,2,4],
    [4,5,6],
    [10,8,-12]
]
var diff = diagonalDiff(algo);
console.log(diff);