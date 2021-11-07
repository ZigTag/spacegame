using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class CameraMovement : MonoBehaviour
{
    // Start is called before the first frame update
    void Start()
    {
        
    }

    // Update is called once per frame
    void Update()
    {
        float horizontalSpeed = .005F;
        float verticalSpeed = .005F;

        float xMouse = Input.mousePosition.x - (Screen.width / 2);
        float yMouse = Input.mousePosition.y - (Screen.height / 2);

        float xMovement = horizontalSpeed * xMouse;
        float yMovement = verticalSpeed * yMouse;
        
        transform.position = new Vector3(xMovement, yMovement, -10);
    }

    void OnMouseDown()
    {
        
    }
}
