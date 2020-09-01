function do_graphs(pageName) {

	function do_bar(resp) {
        var dataPoints = [];
        for (var i = 0; i < resp.data.length && i < 10; i++) {
            dataPoints.push({
                label: resp.data[i].word,
                y: resp.data[i].count
            });
        }
        console.log(dataPoints)

        var chart = new CanvasJS.Chart("chartContainer1", {
            theme: "dark2",
            animationEnabled: true,
            exportEnabled: true,
            axisX: {
                margin: 10,
                labelPlacement: "inside",
                tickPlacement: "inside"
            },
            axisY2: {
                title: "Usage",
                titleFontSize: 14,
                includeZero: true,
            },
            data: [{
                type: "bar",
                axisYType: "secondary",
                yValueFormatString: "###",
                indexLabel: "{y}",
                dataPoints: dataPoints
            }]
        });

        chart.render();
	}
    
    function do_line(resp) {

        var dataPoints = [];
        for (var i = 0; i < resp.data.length && i < 20; i++) {
            dataPoints.push({
                x: i+1,
                y: resp.data[i].count
            });
        }

        var zipfsCurve = [];
        for (var i = 0; i < resp.data.length && i < 20; i++) {
            zipfsCurve.push({
                x: i+1,
                y: dataPoints[0].y/(i+1)
            });
        }

        console.log(dataPoints)

        var chart = new CanvasJS.Chart("chartContainer2", {
            animationEnabled: true,
            zoomEnabled: true,
            theme: "dark2",
            title: {
                text: resp.name
            },

            axisX: {
                title: "Rank",
                valueFormatString: "####",
                interval: 2
            },
            axisY: {
                logarithmic: false,
                includeZero: true,
                title: "Times Used",
                titleFontColor: "#6D78AD",
                lineColor: "#6D78AD",
                gridThickness: 0,
                lineThickness: 1,
            },
            axisY2: {
                title: "Times Used",
                titleFontColor: "#51CDA0",
                logarithmic: false,
                includeZero: true,
                lineColor: "#51CDA0",
                gridThickness: 0,
                lineThickness: 1,
            },
            legend: {
                verticalAlign: "top",
                fontSize: 16,
                dockInsidePlotArea: true
            },
            data: [{
                type: "line",
                xValueFormatString: "##",
                showInLegend: true,
                name: "Zipf Curve",
                dataPoints: zipfsCurve
            },{
                type: "line",
                xValueFormatString: "##",
                axisYType: "secondary",
                showInLegend: true,
                name: "Page Curve",
                dataPoints: dataPoints}
            ]
        });
        chart.render();

    
    }


    var url = "http://73.12.196.137:42069/api/"+pageName;
    $.getJSON(url, function(resp) {
        if (resp.error != null) {
            console.log(resp.error)
            alert("Invalid Page Name")
        } else {
            do_bar(resp);
            do_line(resp);
        }
    });


}



function form_submit() {
    var wikiPage = document.getElementById("wikiPage").value;
    do_graphs(wikiPage)
    return false
}

function random_submit() {
    do_graphs("Special:Random")
}