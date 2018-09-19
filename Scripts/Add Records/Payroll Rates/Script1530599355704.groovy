import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.checkpoint.CheckpointFactory as CheckpointFactory
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as MobileBuiltInKeywords
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testcase.TestCaseFactory as TestCaseFactory
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testdata.TestDataFactory as TestDataFactory
import com.kms.katalon.core.testobject.ObjectRepository as ObjectRepository
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WSBuiltInKeywords
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUiBuiltInKeywords
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('http://mauw2k8sdworx:81/')

WebUI.setText(findTestObject('Screens/Payroll Rates/input_Username'), 'dev@sdworx.com')

WebUI.setText(findTestObject('Screens/Payroll Rates/input_Password'), '1234')

WebUI.click(findTestObject('Screens/Payroll Rates/input_Submit'))

WebUI.maximizeWindow()

WebUI.click(findTestObject('Screens/Payroll Rates/h4_YOUR SETUP'))

WebUI.click(findTestObject('Screens/Payroll Rates/button_Your Setup'))

WebUI.click(findTestObject('Screens/Payroll Rates/a_Your Pay  Benefits'))

WebUI.click(findTestObject('Screens/Payroll Rates/a_Derived Rates'))

WebUI.selectOptionByValue(findTestObject('Screens/Payroll Rates/select_111111 - test981078 - D'), 'e865745f-ca2b-4af8-a580-50285c6b5440', 
    true)

for (def row = 1; row <= findTestData('Payroll Rates').getRowNumbers(); row++) {
    WebUI.click(findTestObject('Screens/Payroll Rates/a_Add Payroll Rate'))

    WebUI.setText(findTestObject('Screens/Payroll Rates/input_RateNumber10'), findTestData('Payroll Rates').getValue(1, row))

    WebUI.setText(findTestObject('Screens/Payroll Rates/input_Description10'), findTestData('Payroll Rates').getValue(2, row))

    WebUI.setText(findTestObject('Screens/Payroll Rates/input_CompanyRateValueperHour0'), findTestData('Payroll Rates').getValue(
            3, row))

    WebUI.selectOptionByValue(findTestObject('Screens/Payroll Rates/select_Please selectR002R009R0 (1)'), 'e3705c5c-bb31-4fa7-9cfa-b7814773d755', 
        true)

    WebUI.setText(findTestObject('Screens/Payroll Rates/input_LinkedCompanyRateMultipl'), findTestData('Payroll Rates').getValue(
            4, row))

    WebUI.click(findTestObject('Screens/Payroll Rates/button_Save (1)'))

}

