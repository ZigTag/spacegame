using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class Space : MonoBehaviour
{
    private float gravitationalConstant = 6.674f * Mathf.Pow(10, -11);
    private float time = 0;
    public GameObject planet;
    public GameObject satellite;
    public LineRenderer prediction;
    public float sma;
    public float eccentricity;
    public float planetMass;
    public int predictionCount;
    public float predictionTimeSpacing;

    Vector3 positionFromTime(float time) {
        float gravitaionalParameter = gravitationalConstant * planetMass;

        float period = 2.0f * Mathf.PI * Mathf.Sqrt(Mathf.Pow(sma, 3) / gravitaionalParameter);

        float meanMotion = 2.0f * Mathf.PI / period;
        float meanAnomaly = meanMotion * time;

        float eccentric_anomaly;

        if (eccentricity < 0.8) {
            eccentric_anomaly = meanAnomaly;
        } else {
            eccentric_anomaly = Mathf.PI;
        }

        float pseudo_true_anomaly = eccentric_anomaly - eccentricity * Mathf.Sin(meanAnomaly) - meanAnomaly;

        float delta = Mathf.Pow(10, -8);
        int i = 0;
        int iCap = 100;

        while ((Mathf.Abs(pseudo_true_anomaly) > delta) && (i < iCap)) {
            eccentric_anomaly = eccentric_anomaly - pseudo_true_anomaly / (1.0f - (eccentricity * Mathf.Cos(eccentric_anomaly)));
            pseudo_true_anomaly = eccentric_anomaly - eccentricity * Mathf.Sin(eccentric_anomaly) - meanAnomaly;
            i += 1;
        }

        // Trust me this is fine
        float true_anomaly = Mathf.Atan2(Mathf.Sqrt(1.0f - (Mathf.Pow(eccentricity, 2))) * Mathf.Sin(eccentric_anomaly), Mathf.Cos(eccentric_anomaly) - eccentricity);

        float distance = sma * (1.0f - (eccentricity * Mathf.Cos(eccentric_anomaly)));

        float x_pos = distance * Mathf.Cos(true_anomaly);
        float y_pos = distance * Mathf.Sin(true_anomaly);

        return new Vector3(x_pos, y_pos, 0);
    }

    Vector3[] generatePrediction() {
        var points = new Vector3[predictionCount];

        for (int i = 0; i < predictionCount; i++) {
            float predTime = i * predictionTimeSpacing + time;

            Vector3 pos = posToParent(positionFromTime(predTime));

            points[i] = pos;
        }

        return points;
    }

    Vector3 posToParent(Vector3 pos) {
        Vector3 planetPos = planet.transform.position;
        return new Vector3(planetPos.x + pos.x, planetPos.y + pos.y, 0);
    }

    // Start is called before the first frame update
    void Start()
    {
        Debug.Log("Orbit System Initialized");
    }

    // Update is called once per frame
    void Update()
    {
        time += Time.deltaTime;

        Vector3 satellitePos = posToParent(positionFromTime(time));

        satellite.transform.position = satellitePos;

        var points = generatePrediction();

        prediction.SetPositions(points);

        // Debug.Log(eccentric_anomaly);
        // Debug.Log(true_anomaly);
        // Debug.Log(distance);
        // Debug.Log(x_pos);
        // Debug.Log(y_pos);
    }
}
