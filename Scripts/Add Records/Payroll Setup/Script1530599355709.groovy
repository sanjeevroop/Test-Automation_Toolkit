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

WebUI.setText(findTestObject('Screens/Payroll Setup/input_Username'), 'dev@sdworx.com')

WebUI.setText(findTestObject('Screens/Payroll Setup/input_Password'), '1234')

WebUI.click(findTestObject('Screens/Payroll Setup/input_Submit'))

WebUI.maximizeWindow()

WebUI.click(findTestObject('Screens/Payroll Setup/h4_YOUR SETUP'))

WebUI.click(findTestObject('Screens/Payroll Setup/button_Admin'))

WebUI.click(findTestObject('Screens/Payroll Setup/a_Payroll  Client Setup'))

WebUI.click(findTestObject('Screens/Payroll Setup/a_Payroll Setup'))

WebUI.selectOptionByValue(findTestObject('Screens/Payroll Setup/select_Please SelectSDWorx-Pra'), '90ad4229-1eae-472b-9d95-2733c25a4367', 
    true)

for (def row = 1; row <= findTestData('Payrollsetup').getRowNumbers(); row++) {

WebUI.click(findTestObject('Screens/Payroll Setup/a_Add Payroll'))

WebUI.selectOptionByValue(findTestObject('Screens/Payroll Setup/select_Please select111119-SDW'), '90ad4229-1eae-472b-9d95-2733c25a4367', 
    true)

WebUI.setText(findTestObject('Screens/Payroll Setup/input_PayrollCompanyNo0'), findTestData('Payrollsetup').getValue(1, row))

WebUI.setText(findTestObject('Screens/Payroll Setup/input_PayrollCompanyName0'), findTestData('Payrollsetup').getValue(2, row))

WebUI.selectOptionByIndex(findTestObject('Screens/Payroll Setup/select_Please SelectYes  No'), findTestData('Payrollsetup').getValue(3, row))

WebUI.selectOptionByIndex(findTestObject('Screens/Payroll Setup/select_Please SelectYes  No_1'), findTestData('Payrollsetup').getValue(4, row))

WebUI.setText(findTestObject('Screens/Payroll Setup/input_ContractAuditNo0'), findTestData('Payrollsetup').getValue(5, row))

WebUI.setText(findTestObject('Screens/Payroll Setup/input_date'), findTestData('Payrollsetup').getValue(6, row))

WebUI.selectOptionByIndex(findTestObject('Screens/Payroll Setup/select_Please SelectYes  No_2'), findTestData('Payrollsetup').getValue(7, row))

WebUI.setText(findTestObject('Screens/Payroll Setup/input_date_1'), findTestData('Payrollsetup').getValue(8, row))

WebUI.selectOptionByIndex(findTestObject('Screens/Payroll Setup/select_Please SelectYes  No_3'), findTestData('Payrollsetup').getValue(9, row))

WebUI.selectOptionByIndex(findTestObject('Screens/Payroll Setup/select_Please SelectYes  No_4'), findTestData('Payrollsetup').getValue(10, row))

WebUI.click(findTestObject('Screens/Payroll Setup/button_Save'))

}

//WebUI.closeBrowser()

